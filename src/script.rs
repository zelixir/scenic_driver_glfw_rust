use byteorder::{NativeEndian, ReadBytesExt};
use comms::*;
use defines::*;
use nanovg::{Color, Gradient, ImagePattern, Paint};
use nanovg_sys::*;
use std::io::Cursor;
use types::*;
use util::*;

pub fn run_scripts<'a, 'ctx: 'tx, 'tx>(
    window_data: &mut WindowData,
    script_id: u32,
    ctx: &Context<'ctx, 'tx>,
    frame: &mut ::nanovg::Frame<'a>,
) {
    if let Some(script) = window_data.get_script(script_id) {
        let script = script.clone();
        let mut read = Cursor::new(script);
        run_script_internal(window_data, &mut read, ctx, frame, None).unwrap();
    }
}

fn run_script_internal<'frame, 'ctx: 'tx, 'tx: 'e, 'e>(
    window_data: &mut WindowData,
    script: &mut impl ReadBytesExt,
    ctx: &'e Context<'ctx, 'tx>,
    frame: &mut ::nanovg::Frame<'frame>,
    curr_paint: Option<Box<Paint + 'e>>,
) -> ::std::io::Result<()> {
    let mut next_paint: Option<Box<Paint + 'e>> = None;
    {
        let op = read_multi!(script, u32)?;
        let raw_ctx = ctx.ctx.raw();
        match op {
            // state control
            OP_PUSH_STATE => unsafe {
                nvgSave(raw_ctx);
            },

            OP_POP_STATE => unsafe {
                nvgRestore(raw_ctx);
            },

            OP_RESET_STATE => unsafe {
                nvgReset(raw_ctx);
            },

            // script control
            OP_RUN_SCRIPT => {
                run_script_internal(window_data, script, ctx, frame, None)?;
            }

            // render styles
            OP_PAINT_LINEAR => {
                next_paint = paint_linear(script);
            }
            OP_PAINT_BOX => {
                next_paint = paint_box(script);
            }
            OP_PAINT_RADIAL => {
                next_paint = paint_radial(script);
            }
            OP_PAINT_IMAGE => {
                next_paint = paint_image(ctx, script);
            }

            OP_ANTI_ALIAS => shape_anti_alias(raw_ctx, script),

            OP_STROKE_WIDTH => shape_width(raw_ctx, script),

            OP_STROKE_COLOR => stroke_color(raw_ctx, script),

            OP_STROKE_PAINT => {
                if let Some(paint) = curr_paint {
                    paint.stroke(ctx.ctx);
                }
            }

            OP_FILL_COLOR => fill_color(ctx.ctx, script),

            OP_FILL_PAINT => {
                if let Some(paint) = curr_paint {
                    paint.fill(ctx.ctx);
                }
            }

            OP_MITER_LIMIT => miter_limit(raw_ctx, script),
            OP_LINE_CAP => line_cap(raw_ctx, script),
            OP_LINE_JOIN => line_join(raw_ctx, script),
            OP_GLOBAL_ALPHA => global_alpha(raw_ctx, script),

            // scissoring
            OP_SCISSOR => scissor(raw_ctx, script),
            OP_INTERSECT_SCISSOR => intersect_scissor(raw_ctx, script),

            OP_RESET_SCISSOR => unsafe {
                nvgResetScissor(raw_ctx);
            },

            // path operations
            OP_PATH_BEGIN => unsafe {
                nvgBeginPath(raw_ctx);
            },

            OP_PATH_MOVE_TO => move_to(raw_ctx, script),
            OP_PATH_LINE_TO => line_to(raw_ctx, script),
            OP_PATH_BEZIER_TO => bezier_to(raw_ctx, script),
            OP_PATH_QUADRATIC_TO => quadratic_to(raw_ctx, script),
            OP_PATH_ARC_TO => arc_to(raw_ctx, script),
            OP_PATH_CLOSE => unsafe {
                nvgClosePath(raw_ctx);
            },
            OP_PATH_WINDING => path_winding(raw_ctx, script),

            OP_FILL => unsafe {
                nvgFill(raw_ctx);
            },
            OP_STROKE => unsafe {
                nvgStroke(raw_ctx);
            },

            OP_TRIANGLE => triangle(raw_ctx, script),
            OP_ARC => arc(raw_ctx, script),
            OP_RECT => rect(raw_ctx, script),
            OP_ROUND_RECT => round_rect(raw_ctx, script),
            OP_ROUND_RECT_VAR => (),
            OP_ELLIPSE => ellipse(raw_ctx, script),
            OP_CIRCLE => circle(raw_ctx, script),
            OP_SECTOR => sector(raw_ctx, script),
            OP_TEXT => text(raw_ctx, script),

            // transform operations
            OP_TX_RESET => unsafe {
                nvgResetTransform(raw_ctx);
            },
            OP_TX_IDENTITY => (),
            OP_TX_MATRIX => tx_matrix(raw_ctx, script),
            OP_TX_TRANSLATE => tx_translate(raw_ctx, script),
            OP_TX_SCALE => tx_scale(raw_ctx, script),
            OP_TX_ROTATE => tx_rotate(raw_ctx, script),
            OP_TX_SKEW_X => tx_skew_x(raw_ctx, script),
            OP_TX_SKEW_Y => tx_skew_y(raw_ctx, script),

            //   // font styles
            OP_FONT => font(raw_ctx, script),
            OP_FONT_BLUR => font_blur(raw_ctx, script),
            OP_FONT_SIZE => font_size(raw_ctx, script),
            OP_TEXT_ALIGN => text_align(raw_ctx, script),
            OP_TEXT_HEIGHT => text_height(raw_ctx, script),
            OP_TERMINATE => return Ok(()),
            _ => {
                send_puts(format!("!!!Unknown script command: {}", op));
                return Ok(());
            }
        }
    }
    run_script_internal(window_data, script, ctx, frame, next_paint)
}
fn paint_linear(script: &mut impl ReadBytesExt) -> Option<Box<Paint>> {
    let (sx, sy, ex, ey, sc, ec) = read_multi!(script, f32, f32, f32, f32, Color, Color).unwrap();

    Some(Box::new(Gradient::Linear {
        start: (sx, sy),
        end: (ex, ey),
        start_color: sc,
        end_color: ec,
    }))
}
fn paint_box(script: &mut impl ReadBytesExt) -> Option<Box<Paint>> {
    let (x, y, w, h, radius, feather, sc, ec) =
        read_multi!(script, f32, f32, f32, f32, f32, f32, Color, Color).unwrap();

    Some(Box::new(Gradient::Box {
        position: (x, y),
        size: (w, h),
        radius: radius,
        feather: feather,
        start_color: sc,
        end_color: ec,
    }))
}
fn paint_radial(script: &mut impl ReadBytesExt) -> Option<Box<Paint>> {
    let (x, y, r_in, r_out, sc, ec) =
        read_multi!(script, f32, f32, f32, f32, Color, Color).unwrap();

    Some(Box::new(Gradient::Radial {
        center: (x, y),
        inner_radius: r_in,
        outer_radius: r_out,
        start_color: sc,
        end_color: ec,
    }))
}
fn paint_image<'ctx: 'tx, 'tx: 'e, 'e>(
    ctx: &'e Context<'ctx, 'tx>,
    script: &mut impl ReadBytesExt,
) -> Option<Box<Paint + 'e>> {
    let (ox, oy, ex, ey, angle, alpha, key_size) =
        read_multi!(script, f32, f32, f32, f32, f32, f32, u32).unwrap();
    let key = read_string(script, key_size as usize);
    if let Some(image) = ctx.textures.get(&key) {
        Some(Box::new(ImagePattern {
            image: image,
            origin: (ox, oy),
            size: (ex, ey),
            angle: angle,
            alpha: alpha,
        }))
    } else {
        send_cache_miss(key);
        None
    }
}
fn shape_anti_alias(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgShapeAntiAlias(ctx, read_multi!(script, i32).unwrap());
    }
}
fn shape_width(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgStrokeWidth(ctx, read_multi!(script, f32).unwrap());
    }
}
fn stroke_color(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (r, g, b, a) = read_multi!(script, u32, u32, u32, u32).unwrap();
    unsafe {
        nvgStrokeColor(ctx, nvgRGBA(r as u8, g as u8, b as u8, a as u8));
    }
}
fn fill_color(ctx: &NanoContext, script: &mut impl ReadBytesExt) {
    read_multi!(script, Color).unwrap().fill(ctx)
}
fn miter_limit(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe { nvgMiterLimit(ctx, read_multi!(script, f32).unwrap()) }
}
fn line_cap(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe { nvgLineCap(ctx, read_multi!(script, i32).unwrap()) }
}
fn line_join(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgLineCap(
            ctx,
            match read_multi!(script, i32).unwrap() {
                0 => NVGlineCap::NVG_MITER as i32,
                1 => NVGlineCap::NVG_ROUND as i32,
                2 => NVGlineCap::NVG_BEVEL as i32,
                i => i,
            },
        )
    }
}
fn global_alpha(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe { nvgGlobalAlpha(ctx, read_multi!(script, f32).unwrap()) }
}
fn scissor(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (w, h) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgScissor(ctx, 0f32, 0f32, w, h);
    }
}
fn intersect_scissor(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (w, h) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgIntersectScissor(ctx, 0f32, 0f32, w, h);
    }
}
fn move_to(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x, y) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgMoveTo(ctx, x, y);
    }
}
fn line_to(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x, y) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgLineTo(ctx, x, y);
    }
}
fn bezier_to(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (c1x, c1y, c2x, c2y, x, y) = read_multi!(script, f32, f32, f32, f32, f32, f32).unwrap();
    unsafe {
        nvgBezierTo(ctx, c1x, c1y, c2x, c2y, x, y);
    }
}
fn quadratic_to(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x1, y1, x2, y2) = read_multi!(script, f32, f32, f32, f32).unwrap();
    unsafe {
        nvgQuadTo(ctx, x1, y1, x2, y2);
    }
}
fn arc_to(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x1, y1, x2, y2, radius) = read_multi!(script, f32, f32, f32, f32, f32).unwrap();
    unsafe {
        nvgArcTo(ctx, x1, y1, x2, y2, radius);
    }
}
fn path_winding(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgPathWinding(
            ctx,
            if read_multi!(script, bool).unwrap() {
                NVGsolidity::NVG_SOLID
            } else {
                NVGsolidity::NVG_HOLE
            }.bits(),
        );
    }
}
fn triangle(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x0, y0, x1, y1, x2, y2) = read_multi!(script, f32, f32, f32, f32, f32, f32).unwrap();
    unsafe {
        nvgMoveTo(ctx, x0, y0);
        nvgLineTo(ctx, x1, y1);
        nvgLineTo(ctx, x2, y2);
        nvgClosePath(ctx);
    }
}
fn rect(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (w, h) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgRect(ctx, 0f32, 0f32, w, h);
    }
}
fn round_rect(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (w, h, r) = read_multi!(script, f32, f32, f32).unwrap();
    unsafe {
        nvgRoundedRect(ctx, 0f32, 0f32, w, h, r);
    }
}
fn ellipse(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (rx, ry) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgEllipse(ctx, 0f32, 0f32, rx, ry);
    }
}
fn circle(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgCircle(ctx, 0f32, 0f32, read_multi!(script, f32).unwrap());
    }
}
fn arc(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (radius, start, finish) = read_multi!(script, f32, f32, f32).unwrap();
    let mut angle = finish - start;
    angle = if angle > TAU { TAU } else { angle };
    angle = if angle < -TAU { -TAU } else { angle };
    let segment_count = (radius.log2() * angle.abs() * 2.0) as u32;
    let increment: f32 = angle / segment_count as f32;
    let mut a: f32 = start;

    unsafe {
        for i in 0..segment_count {
            let px = a.cos() * radius;
            let py = a.sin() * radius;

            if i == 0 {
                nvgMoveTo(ctx, px, py);
            } else {
                nvgLineTo(ctx, px, py);
            }
            a += increment;
        }
    }
}
fn sector(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    arc(ctx, script);
    unsafe {
        nvgLineTo(ctx, 0f32, 0f32);
        nvgClosePath(ctx);
    }
}
fn text(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let len = read_multi!(script, usize).unwrap();
    let text = read_bytes(script, len);

    unsafe {
        let mut height = 0f32;
        let mut start: *const i8 = text.as_slice().as_ptr() as *const i8;
        let end = start.offset(len as isize);
        nvgTextMetrics(
            ctx,
            ::std::ptr::null_mut(),
            ::std::ptr::null_mut(),
            &mut height,
        );
        let x = 0f32;
        let mut y = 0f32;
        let mut rows_raw = [0u8; ::std::mem::size_of::<NVGtextRow>() * 3];
        let rows: *mut NVGtextRow = rows_raw.as_mut_ptr() as *mut NVGtextRow;
        loop {
            let nrows = nvgTextBreakLines(ctx, start, end, 1000f32, rows, 3);
            for i in 0..nrows {
                let row = rows.offset(i as isize - 1);
                nvgText(ctx, x, y, (*row).start, (*row).end);
                y += height;
            }
            start = (*rows.offset(nrows as isize - 1)).next;
        }
    }
}

fn tx_rotate(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgRotate(ctx, read_multi!(script, f32).unwrap());
    }
}
fn tx_translate(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x, y) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgTranslate(ctx, x, y);
    }
}
fn tx_scale(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (x, y) = read_multi!(script, f32, f32).unwrap();
    unsafe {
        nvgScale(ctx, x, y);
    }
}
fn tx_skew_x(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgSkewX(ctx, read_multi!(script, f32).unwrap());
    }
}
fn tx_skew_y(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgSkewY(ctx, read_multi!(script, f32).unwrap());
    }
}
fn tx_matrix(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let (a, b, c, d, e, f) = read_multi!(script, f32, f32, f32, f32, f32, f32).unwrap();
    unsafe {
        nvgTransform(ctx, a, b, c, d, e, f);
    }
}

fn font(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    let len = read_multi!(script, usize).unwrap();
    let text = read_string(script, len);
    unsafe {
        let font_id = nvgFindFont(ctx, text.as_bytes().as_ptr() as *const i8);
        if font_id >= 0 {
            nvgFontFaceId(ctx, font_id);
        } else {
            send_font_miss(text);
        }
    }
}
fn font_blur(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgFontBlur(ctx, read_multi!(script, f32).unwrap());
    }
}
fn font_size(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgFontSize(ctx, read_multi!(script, f32).unwrap());
    }
}
fn text_align(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgTextAlign(ctx, read_multi!(script, i32).unwrap());
    }
}
fn text_height(ctx: *mut NVGcontext, script: &mut impl ReadBytesExt) {
    unsafe {
        nvgTextLineHeight(ctx, read_multi!(script, f32).unwrap());
    }
}
