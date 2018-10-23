// Copyright 2018 The Synthesizer IO Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Widget for oscilloscope display.

use direct2d::enums::BitmapInterpolationMode;
use direct2d::image::Bitmap;
use direct2d::math::SizeU;
use direct2d::RenderTarget;

use dxgi::Format;
use winapi::um::dcommon::D2D_SIZE_U;
use winapi::shared::basetsd::UINT32;

use xi_win_ui::{BoxConstraints, LayoutCtx, LayoutResult};
use xi_win_ui::{Id, Geometry, PaintCtx, UiInner, Widget};

use synthesize_scope as s;

pub struct Scope {
    // I might want to call the data structure ScopeBuf or some such,
    // too many name collisions :/
    s: s::Scope,
}

impl Widget for Scope {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, geom: &Geometry) {
        let rt = paint_ctx.render_target();
        let w = 640;
        let h = 480;
        let data = self.s.as_rgba();
        let b = Bitmap::create(rt)
            .with_raw_data(
                SizeU(D2D_SIZE_U {
                    width: w as UINT32, height: h as UINT32
                }),
                &data,
                w as UINT32 * 4)
            .with_format(Format::R8G8B8A8Unorm)
            .build().expect("error creating bitmap");
        let height = geom.size.1.min(0.75 * geom.size.0);
        let width = height * (1.0 / 0.75);
        let x0 = geom.pos.0;
        let y0 = geom.pos.1;
        rt.draw_bitmap(
            &b,
            (x0 + geom.size.0 - width, y0, x0 + geom.size.0, y0 + height),
            1.0,
            BitmapInterpolationMode::Linear,
            (0.0, 0.0, w as f32, h as f32),
        );
    }

    fn layout(&mut self, bc: &BoxConstraints, _children: &[Id], _size: Option<(f32, f32)>,
        _ctx: &mut LayoutCtx) -> LayoutResult
    {
        let size = bc.constrain((100.0, 100.0));
        //self.size = size;
        LayoutResult::Size(size)
    }
}

impl Scope {
    pub fn new() -> Scope {
        let s = s::Scope::new(640, 480);
        let mut result = Scope { s };
        result.draw_test_pattern();
        result
    }

    pub fn ui(self, ctx: &mut UiInner) -> Id {
        ctx.add(self, &[])
    }

    fn draw_test_pattern(&mut self) {
        let mut xylast = None;
        // sinewave!
        for i in 0..1001 {
            let h = (i as f32) * 0.001;
            let x = 640.0 * h;
            let y = 240.0 + 200.0 * (h * 50.0).sin();
            if let Some((xlast, ylast)) = xylast {
                self.s.add_line(xlast, ylast, x, y, 1.0, 2.0);
            }
            xylast = Some((x, y));
        }

    }
}