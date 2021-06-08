//! The [swizzle](self) module contains the implementation for vector
//! swizzling. This implements a set of getters and setters for every
//! permutation of vector components described by the three sets:
//! `{x, y, z, w}, {r, g, b, a}, {s, t, p, q}`. During compilation these
//! swizzle methods will be translated into their GLSL equivalent.
use super::math::{V2, V3, V4};

use num::Num;

impl<T: Num + Clone> V2<T> {
  pub fn x(&self) -> T { self.0.clone() }
  pub fn set_x(&mut self, value: T) { self.0 = value; }
  pub fn y(&self) -> T { self.1.clone() }
  pub fn set_y(&mut self, value: T) { self.1 = value; }
  pub fn xx(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn xy(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_xy(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn yx(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_yx(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn yy(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn r(&self) -> T { self.0.clone() }
  pub fn set_r(&mut self, value: T) { self.0 = value; }
  pub fn g(&self) -> T { self.1.clone() }
  pub fn set_g(&mut self, value: T) { self.1 = value; }
  pub fn rr(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn rg(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_rg(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn gr(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_gr(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn gg(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn s(&self) -> T { self.0.clone() }
  pub fn set_s(&mut self, value: T) { self.0 = value; }
  pub fn t(&self) -> T { self.1.clone() }
  pub fn set_t(&mut self, value: T) { self.1 = value; }
  pub fn ss(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn st(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_st(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn ts(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_ts(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn tt(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
}

impl<T: Num + Clone> V3<T> {
  pub fn x(&self) -> T { self.0.clone() }
  pub fn set_x(&mut self, value: T) { self.0 = value; }
  pub fn y(&self) -> T { self.1.clone() }
  pub fn set_y(&mut self, value: T) { self.1 = value; }
  pub fn z(&self) -> T { self.2.clone() }
  pub fn set_z(&mut self, value: T) { self.2 = value; }
  pub fn xx(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn xy(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_xy(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn xz(&self) -> V2<T> { V2(self.0.clone(), self.2.clone()) }
  pub fn set_xz(&mut self, value: V2<T>) { self.0 = value.0; self.2 = value.1; }
  pub fn yx(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_yx(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn yy(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn yz(&self) -> V2<T> { V2(self.1.clone(), self.2.clone()) }
  pub fn set_yz(&mut self, value: V2<T>) { self.1 = value.0; self.2 = value.1; }
  pub fn zx(&self) -> V2<T> { V2(self.2.clone(), self.0.clone()) }
  pub fn set_zx(&mut self, value: V2<T>) { self.2 = value.0; self.0 = value.1; }
  pub fn zy(&self) -> V2<T> { V2(self.2.clone(), self.1.clone()) }
  pub fn set_zy(&mut self, value: V2<T>) { self.2 = value.0; self.1 = value.1; }
  pub fn zz(&self) -> V2<T> { V2(self.2.clone(), self.2.clone()) }
  pub fn xxx(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn xxy(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn xxz(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn xyx(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn xyy(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn xyz(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_xyz(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn xzx(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn xzy(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_xzy(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn xzz(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn yxx(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn yxy(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn yxz(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_yxz(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn yyx(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn yyy(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn yyz(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn yzx(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_yzx(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn yzy(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn yzz(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn zxx(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn zxy(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_zxy(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn zxz(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn zyx(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_zyx(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn zyy(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn zyz(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn zzx(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn zzy(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn zzz(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn r(&self) -> T { self.0.clone() }
  pub fn set_r(&mut self, value: T) { self.0 = value; }
  pub fn g(&self) -> T { self.1.clone() }
  pub fn set_g(&mut self, value: T) { self.1 = value; }
  pub fn b(&self) -> T { self.2.clone() }
  pub fn set_b(&mut self, value: T) { self.2 = value; }
  pub fn rr(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn rg(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_rg(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn rb(&self) -> V2<T> { V2(self.0.clone(), self.2.clone()) }
  pub fn set_rb(&mut self, value: V2<T>) { self.0 = value.0; self.2 = value.1; }
  pub fn gr(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_gr(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn gg(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn gb(&self) -> V2<T> { V2(self.1.clone(), self.2.clone()) }
  pub fn set_gb(&mut self, value: V2<T>) { self.1 = value.0; self.2 = value.1; }
  pub fn br(&self) -> V2<T> { V2(self.2.clone(), self.0.clone()) }
  pub fn set_br(&mut self, value: V2<T>) { self.2 = value.0; self.0 = value.1; }
  pub fn bg(&self) -> V2<T> { V2(self.2.clone(), self.1.clone()) }
  pub fn set_bg(&mut self, value: V2<T>) { self.2 = value.0; self.1 = value.1; }
  pub fn bb(&self) -> V2<T> { V2(self.2.clone(), self.2.clone()) }
  pub fn rrr(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn rrg(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn rrb(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn rgr(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn rgg(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn rgb(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_rgb(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn rbr(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn rbg(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_rbg(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn rbb(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn grr(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn grg(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn grb(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_grb(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn ggr(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn ggg(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn ggb(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn gbr(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_gbr(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn gbg(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn gbb(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn brr(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn brg(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_brg(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn brb(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn bgr(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_bgr(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn bgg(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn bgb(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn bbr(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn bbg(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn bbb(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn s(&self) -> T { self.0.clone() }
  pub fn set_s(&mut self, value: T) { self.0 = value; }
  pub fn t(&self) -> T { self.1.clone() }
  pub fn set_t(&mut self, value: T) { self.1 = value; }
  pub fn p(&self) -> T { self.2.clone() }
  pub fn set_p(&mut self, value: T) { self.2 = value; }
  pub fn ss(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn st(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_st(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn sp(&self) -> V2<T> { V2(self.0.clone(), self.2.clone()) }
  pub fn set_sp(&mut self, value: V2<T>) { self.0 = value.0; self.2 = value.1; }
  pub fn ts(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_ts(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn tt(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn tp(&self) -> V2<T> { V2(self.1.clone(), self.2.clone()) }
  pub fn set_tp(&mut self, value: V2<T>) { self.1 = value.0; self.2 = value.1; }
  pub fn ps(&self) -> V2<T> { V2(self.2.clone(), self.0.clone()) }
  pub fn set_ps(&mut self, value: V2<T>) { self.2 = value.0; self.0 = value.1; }
  pub fn pt(&self) -> V2<T> { V2(self.2.clone(), self.1.clone()) }
  pub fn set_pt(&mut self, value: V2<T>) { self.2 = value.0; self.1 = value.1; }
  pub fn pp(&self) -> V2<T> { V2(self.2.clone(), self.2.clone()) }
  pub fn sss(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn sst(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn ssp(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn sts(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn stt(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn stp(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_stp(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn sps(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn spt(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_spt(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn spp(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn tss(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn tst(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn tsp(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_tsp(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn tts(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn ttt(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn ttp(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn tps(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_tps(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn tpt(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn tpp(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn pss(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn pst(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_pst(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn psp(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn pts(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_pts(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn ptt(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn ptp(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn pps(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn ppt(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn ppp(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.2.clone()) }
}

impl<T: Num + Clone> V4<T> {
  pub fn x(&self) -> T { self.0.clone() }
  pub fn set_x(&mut self, value: T) { self.0 = value; }
  pub fn y(&self) -> T { self.1.clone() }
  pub fn set_y(&mut self, value: T) { self.1 = value; }
  pub fn z(&self) -> T { self.2.clone() }
  pub fn set_z(&mut self, value: T) { self.2 = value; }
  pub fn w(&self) -> T { self.3.clone() }
  pub fn set_w(&mut self, value: T) { self.3 = value; }
  pub fn xx(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn xy(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_xy(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn xz(&self) -> V2<T> { V2(self.0.clone(), self.2.clone()) }
  pub fn set_xz(&mut self, value: V2<T>) { self.0 = value.0; self.2 = value.1; }
  pub fn xw(&self) -> V2<T> { V2(self.0.clone(), self.3.clone()) }
  pub fn set_xw(&mut self, value: V2<T>) { self.0 = value.0; self.3 = value.1; }
  pub fn yx(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_yx(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn yy(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn yz(&self) -> V2<T> { V2(self.1.clone(), self.2.clone()) }
  pub fn set_yz(&mut self, value: V2<T>) { self.1 = value.0; self.2 = value.1; }
  pub fn yw(&self) -> V2<T> { V2(self.1.clone(), self.3.clone()) }
  pub fn set_yw(&mut self, value: V2<T>) { self.1 = value.0; self.3 = value.1; }
  pub fn zx(&self) -> V2<T> { V2(self.2.clone(), self.0.clone()) }
  pub fn set_zx(&mut self, value: V2<T>) { self.2 = value.0; self.0 = value.1; }
  pub fn zy(&self) -> V2<T> { V2(self.2.clone(), self.1.clone()) }
  pub fn set_zy(&mut self, value: V2<T>) { self.2 = value.0; self.1 = value.1; }
  pub fn zz(&self) -> V2<T> { V2(self.2.clone(), self.2.clone()) }
  pub fn zw(&self) -> V2<T> { V2(self.2.clone(), self.3.clone()) }
  pub fn set_zw(&mut self, value: V2<T>) { self.2 = value.0; self.3 = value.1; }
  pub fn wx(&self) -> V2<T> { V2(self.3.clone(), self.0.clone()) }
  pub fn set_wx(&mut self, value: V2<T>) { self.3 = value.0; self.0 = value.1; }
  pub fn wy(&self) -> V2<T> { V2(self.3.clone(), self.1.clone()) }
  pub fn set_wy(&mut self, value: V2<T>) { self.3 = value.0; self.1 = value.1; }
  pub fn wz(&self) -> V2<T> { V2(self.3.clone(), self.2.clone()) }
  pub fn set_wz(&mut self, value: V2<T>) { self.3 = value.0; self.2 = value.1; }
  pub fn ww(&self) -> V2<T> { V2(self.3.clone(), self.3.clone()) }
  pub fn xxx(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn xxy(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn xxz(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn xxw(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn xyx(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn xyy(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn xyz(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_xyz(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn xyw(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_xyw(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.3 = value.2; }
  pub fn xzx(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn xzy(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_xzy(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn xzz(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn xzw(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_xzw(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.3 = value.2; }
  pub fn xwx(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn xwy(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_xwy(&mut self, value: V3<T>) { self.0 = value.0; self.3 = value.1; self.1 = value.2; }
  pub fn xwz(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_xwz(&mut self, value: V3<T>) { self.0 = value.0; self.3 = value.1; self.2 = value.2; }
  pub fn xww(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn yxx(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn yxy(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn yxz(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_yxz(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn yxw(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_yxw(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.3 = value.2; }
  pub fn yyx(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn yyy(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn yyz(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn yyw(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn yzx(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_yzx(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn yzy(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn yzz(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn yzw(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_yzw(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.3 = value.2; }
  pub fn ywx(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_ywx(&mut self, value: V3<T>) { self.1 = value.0; self.3 = value.1; self.0 = value.2; }
  pub fn ywy(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn ywz(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_ywz(&mut self, value: V3<T>) { self.1 = value.0; self.3 = value.1; self.2 = value.2; }
  pub fn yww(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn zxx(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn zxy(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_zxy(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn zxz(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn zxw(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_zxw(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.3 = value.2; }
  pub fn zyx(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_zyx(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn zyy(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn zyz(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn zyw(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_zyw(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.3 = value.2; }
  pub fn zzx(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn zzy(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn zzz(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn zzw(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn zwx(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_zwx(&mut self, value: V3<T>) { self.2 = value.0; self.3 = value.1; self.0 = value.2; }
  pub fn zwy(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_zwy(&mut self, value: V3<T>) { self.2 = value.0; self.3 = value.1; self.1 = value.2; }
  pub fn zwz(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn zww(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn wxx(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn wxy(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_wxy(&mut self, value: V3<T>) { self.3 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn wxz(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_wxz(&mut self, value: V3<T>) { self.3 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn wxw(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn wyx(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_wyx(&mut self, value: V3<T>) { self.3 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn wyy(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn wyz(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_wyz(&mut self, value: V3<T>) { self.3 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn wyw(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn wzx(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_wzx(&mut self, value: V3<T>) { self.3 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn wzy(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_wzy(&mut self, value: V3<T>) { self.3 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn wzz(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn wzw(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn wwx(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn wwy(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn wwz(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn www(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn xxxx(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn xxxy(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn xxxz(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn xxxw(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn xxyx(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn xxyy(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn xxyz(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn xxyw(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn xxzx(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn xxzy(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn xxzz(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn xxzw(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn xxwx(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn xxwy(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn xxwz(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn xxww(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn xyxx(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn xyxy(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn xyxz(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn xyxw(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn xyyx(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn xyyy(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn xyyz(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn xyyw(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn xyzx(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn xyzy(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn xyzz(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn xyzw(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_xyzw(&mut self, value: V4<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; self.3 = value.3; }
  pub fn xywx(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn xywy(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn xywz(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_xywz(&mut self, value: V4<T>) { self.0 = value.0; self.1 = value.1; self.3 = value.2; self.2 = value.3; }
  pub fn xyww(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn xzxx(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn xzxy(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn xzxz(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn xzxw(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn xzyx(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn xzyy(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn xzyz(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn xzyw(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_xzyw(&mut self, value: V4<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; self.3 = value.3; }
  pub fn xzzx(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn xzzy(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn xzzz(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn xzzw(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn xzwx(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn xzwy(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_xzwy(&mut self, value: V4<T>) { self.0 = value.0; self.2 = value.1; self.3 = value.2; self.1 = value.3; }
  pub fn xzwz(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn xzww(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn xwxx(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn xwxy(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn xwxz(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn xwxw(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn xwyx(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn xwyy(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn xwyz(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_xwyz(&mut self, value: V4<T>) { self.0 = value.0; self.3 = value.1; self.1 = value.2; self.2 = value.3; }
  pub fn xwyw(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn xwzx(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn xwzy(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_xwzy(&mut self, value: V4<T>) { self.0 = value.0; self.3 = value.1; self.2 = value.2; self.1 = value.3; }
  pub fn xwzz(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn xwzw(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn xwwx(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn xwwy(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn xwwz(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn xwww(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn yxxx(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn yxxy(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn yxxz(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn yxxw(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn yxyx(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn yxyy(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn yxyz(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn yxyw(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn yxzx(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn yxzy(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn yxzz(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn yxzw(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_yxzw(&mut self, value: V4<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; self.3 = value.3; }
  pub fn yxwx(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn yxwy(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn yxwz(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_yxwz(&mut self, value: V4<T>) { self.1 = value.0; self.0 = value.1; self.3 = value.2; self.2 = value.3; }
  pub fn yxww(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn yyxx(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn yyxy(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn yyxz(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn yyxw(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn yyyx(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn yyyy(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn yyyz(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn yyyw(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn yyzx(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn yyzy(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn yyzz(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn yyzw(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn yywx(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn yywy(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn yywz(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn yyww(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn yzxx(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn yzxy(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn yzxz(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn yzxw(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_yzxw(&mut self, value: V4<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; self.3 = value.3; }
  pub fn yzyx(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn yzyy(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn yzyz(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn yzyw(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn yzzx(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn yzzy(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn yzzz(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn yzzw(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn yzwx(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_yzwx(&mut self, value: V4<T>) { self.1 = value.0; self.2 = value.1; self.3 = value.2; self.0 = value.3; }
  pub fn yzwy(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn yzwz(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn yzww(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn ywxx(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn ywxy(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn ywxz(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_ywxz(&mut self, value: V4<T>) { self.1 = value.0; self.3 = value.1; self.0 = value.2; self.2 = value.3; }
  pub fn ywxw(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn ywyx(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn ywyy(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn ywyz(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn ywyw(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn ywzx(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_ywzx(&mut self, value: V4<T>) { self.1 = value.0; self.3 = value.1; self.2 = value.2; self.0 = value.3; }
  pub fn ywzy(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn ywzz(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn ywzw(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn ywwx(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn ywwy(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn ywwz(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn ywww(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn zxxx(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn zxxy(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn zxxz(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn zxxw(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn zxyx(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn zxyy(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn zxyz(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn zxyw(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_zxyw(&mut self, value: V4<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; self.3 = value.3; }
  pub fn zxzx(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn zxzy(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn zxzz(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn zxzw(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn zxwx(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn zxwy(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_zxwy(&mut self, value: V4<T>) { self.2 = value.0; self.0 = value.1; self.3 = value.2; self.1 = value.3; }
  pub fn zxwz(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn zxww(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn zyxx(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn zyxy(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn zyxz(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn zyxw(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_zyxw(&mut self, value: V4<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; self.3 = value.3; }
  pub fn zyyx(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn zyyy(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn zyyz(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn zyyw(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn zyzx(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn zyzy(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn zyzz(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn zyzw(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn zywx(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_zywx(&mut self, value: V4<T>) { self.2 = value.0; self.1 = value.1; self.3 = value.2; self.0 = value.3; }
  pub fn zywy(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn zywz(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn zyww(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn zzxx(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn zzxy(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn zzxz(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn zzxw(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn zzyx(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn zzyy(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn zzyz(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn zzyw(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn zzzx(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn zzzy(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn zzzz(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn zzzw(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn zzwx(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn zzwy(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn zzwz(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn zzww(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn zwxx(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn zwxy(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_zwxy(&mut self, value: V4<T>) { self.2 = value.0; self.3 = value.1; self.0 = value.2; self.1 = value.3; }
  pub fn zwxz(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn zwxw(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn zwyx(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_zwyx(&mut self, value: V4<T>) { self.2 = value.0; self.3 = value.1; self.1 = value.2; self.0 = value.3; }
  pub fn zwyy(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn zwyz(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn zwyw(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn zwzx(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn zwzy(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn zwzz(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn zwzw(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn zwwx(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn zwwy(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn zwwz(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn zwww(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn wxxx(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn wxxy(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn wxxz(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn wxxw(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn wxyx(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn wxyy(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn wxyz(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_wxyz(&mut self, value: V4<T>) { self.3 = value.0; self.0 = value.1; self.1 = value.2; self.2 = value.3; }
  pub fn wxyw(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn wxzx(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn wxzy(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_wxzy(&mut self, value: V4<T>) { self.3 = value.0; self.0 = value.1; self.2 = value.2; self.1 = value.3; }
  pub fn wxzz(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn wxzw(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn wxwx(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn wxwy(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn wxwz(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn wxww(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn wyxx(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn wyxy(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn wyxz(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_wyxz(&mut self, value: V4<T>) { self.3 = value.0; self.1 = value.1; self.0 = value.2; self.2 = value.3; }
  pub fn wyxw(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn wyyx(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn wyyy(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn wyyz(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn wyyw(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn wyzx(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_wyzx(&mut self, value: V4<T>) { self.3 = value.0; self.1 = value.1; self.2 = value.2; self.0 = value.3; }
  pub fn wyzy(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn wyzz(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn wyzw(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn wywx(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn wywy(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn wywz(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn wyww(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn wzxx(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn wzxy(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_wzxy(&mut self, value: V4<T>) { self.3 = value.0; self.2 = value.1; self.0 = value.2; self.1 = value.3; }
  pub fn wzxz(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn wzxw(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn wzyx(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_wzyx(&mut self, value: V4<T>) { self.3 = value.0; self.2 = value.1; self.1 = value.2; self.0 = value.3; }
  pub fn wzyy(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn wzyz(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn wzyw(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn wzzx(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn wzzy(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn wzzz(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn wzzw(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn wzwx(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn wzwy(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn wzwz(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn wzww(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn wwxx(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn wwxy(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn wwxz(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn wwxw(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn wwyx(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn wwyy(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn wwyz(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn wwyw(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn wwzx(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn wwzy(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn wwzz(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn wwzw(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn wwwx(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn wwwy(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn wwwz(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn wwww(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn r(&self) -> T { self.0.clone() }
  pub fn set_r(&mut self, value: T) { self.0 = value; }
  pub fn g(&self) -> T { self.1.clone() }
  pub fn set_g(&mut self, value: T) { self.1 = value; }
  pub fn b(&self) -> T { self.2.clone() }
  pub fn set_b(&mut self, value: T) { self.2 = value; }
  pub fn a(&self) -> T { self.3.clone() }
  pub fn set_a(&mut self, value: T) { self.3 = value; }
  pub fn rr(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn rg(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_rg(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn rb(&self) -> V2<T> { V2(self.0.clone(), self.2.clone()) }
  pub fn set_rb(&mut self, value: V2<T>) { self.0 = value.0; self.2 = value.1; }
  pub fn ra(&self) -> V2<T> { V2(self.0.clone(), self.3.clone()) }
  pub fn set_ra(&mut self, value: V2<T>) { self.0 = value.0; self.3 = value.1; }
  pub fn gr(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_gr(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn gg(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn gb(&self) -> V2<T> { V2(self.1.clone(), self.2.clone()) }
  pub fn set_gb(&mut self, value: V2<T>) { self.1 = value.0; self.2 = value.1; }
  pub fn ga(&self) -> V2<T> { V2(self.1.clone(), self.3.clone()) }
  pub fn set_ga(&mut self, value: V2<T>) { self.1 = value.0; self.3 = value.1; }
  pub fn br(&self) -> V2<T> { V2(self.2.clone(), self.0.clone()) }
  pub fn set_br(&mut self, value: V2<T>) { self.2 = value.0; self.0 = value.1; }
  pub fn bg(&self) -> V2<T> { V2(self.2.clone(), self.1.clone()) }
  pub fn set_bg(&mut self, value: V2<T>) { self.2 = value.0; self.1 = value.1; }
  pub fn bb(&self) -> V2<T> { V2(self.2.clone(), self.2.clone()) }
  pub fn ba(&self) -> V2<T> { V2(self.2.clone(), self.3.clone()) }
  pub fn set_ba(&mut self, value: V2<T>) { self.2 = value.0; self.3 = value.1; }
  pub fn ar(&self) -> V2<T> { V2(self.3.clone(), self.0.clone()) }
  pub fn set_ar(&mut self, value: V2<T>) { self.3 = value.0; self.0 = value.1; }
  pub fn ag(&self) -> V2<T> { V2(self.3.clone(), self.1.clone()) }
  pub fn set_ag(&mut self, value: V2<T>) { self.3 = value.0; self.1 = value.1; }
  pub fn ab(&self) -> V2<T> { V2(self.3.clone(), self.2.clone()) }
  pub fn set_ab(&mut self, value: V2<T>) { self.3 = value.0; self.2 = value.1; }
  pub fn aa(&self) -> V2<T> { V2(self.3.clone(), self.3.clone()) }
  pub fn rrr(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn rrg(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn rrb(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn rra(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn rgr(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn rgg(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn rgb(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_rgb(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn rga(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_rga(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.3 = value.2; }
  pub fn rbr(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn rbg(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_rbg(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn rbb(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn rba(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_rba(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.3 = value.2; }
  pub fn rar(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn rag(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_rag(&mut self, value: V3<T>) { self.0 = value.0; self.3 = value.1; self.1 = value.2; }
  pub fn rab(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_rab(&mut self, value: V3<T>) { self.0 = value.0; self.3 = value.1; self.2 = value.2; }
  pub fn raa(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn grr(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn grg(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn grb(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_grb(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn gra(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_gra(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.3 = value.2; }
  pub fn ggr(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn ggg(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn ggb(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn gga(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn gbr(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_gbr(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn gbg(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn gbb(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn gba(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_gba(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.3 = value.2; }
  pub fn gar(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_gar(&mut self, value: V3<T>) { self.1 = value.0; self.3 = value.1; self.0 = value.2; }
  pub fn gag(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn gab(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_gab(&mut self, value: V3<T>) { self.1 = value.0; self.3 = value.1; self.2 = value.2; }
  pub fn gaa(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn brr(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn brg(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_brg(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn brb(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn bra(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_bra(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.3 = value.2; }
  pub fn bgr(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_bgr(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn bgg(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn bgb(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn bga(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_bga(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.3 = value.2; }
  pub fn bbr(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn bbg(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn bbb(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn bba(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn bar(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_bar(&mut self, value: V3<T>) { self.2 = value.0; self.3 = value.1; self.0 = value.2; }
  pub fn bag(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_bag(&mut self, value: V3<T>) { self.2 = value.0; self.3 = value.1; self.1 = value.2; }
  pub fn bab(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn baa(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn arr(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn arg(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_arg(&mut self, value: V3<T>) { self.3 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn arb(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_arb(&mut self, value: V3<T>) { self.3 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn ara(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn agr(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_agr(&mut self, value: V3<T>) { self.3 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn agg(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn agb(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_agb(&mut self, value: V3<T>) { self.3 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn aga(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn abr(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_abr(&mut self, value: V3<T>) { self.3 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn abg(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_abg(&mut self, value: V3<T>) { self.3 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn abb(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn aba(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn aar(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn aag(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn aab(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn aaa(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn rrrr(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn rrrg(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn rrrb(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn rrra(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn rrgr(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn rrgg(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn rrgb(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn rrga(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn rrbr(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn rrbg(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn rrbb(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn rrba(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn rrar(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn rrag(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn rrab(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn rraa(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn rgrr(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn rgrg(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn rgrb(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn rgra(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn rggr(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn rggg(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn rggb(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn rgga(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn rgbr(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn rgbg(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn rgbb(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn rgba(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_rgba(&mut self, value: V4<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; self.3 = value.3; }
  pub fn rgar(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn rgag(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn rgab(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_rgab(&mut self, value: V4<T>) { self.0 = value.0; self.1 = value.1; self.3 = value.2; self.2 = value.3; }
  pub fn rgaa(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn rbrr(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn rbrg(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn rbrb(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn rbra(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn rbgr(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn rbgg(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn rbgb(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn rbga(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_rbga(&mut self, value: V4<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; self.3 = value.3; }
  pub fn rbbr(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn rbbg(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn rbbb(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn rbba(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn rbar(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn rbag(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_rbag(&mut self, value: V4<T>) { self.0 = value.0; self.2 = value.1; self.3 = value.2; self.1 = value.3; }
  pub fn rbab(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn rbaa(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn rarr(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn rarg(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn rarb(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn rara(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn ragr(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn ragg(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn ragb(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_ragb(&mut self, value: V4<T>) { self.0 = value.0; self.3 = value.1; self.1 = value.2; self.2 = value.3; }
  pub fn raga(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn rabr(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn rabg(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_rabg(&mut self, value: V4<T>) { self.0 = value.0; self.3 = value.1; self.2 = value.2; self.1 = value.3; }
  pub fn rabb(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn raba(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn raar(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn raag(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn raab(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn raaa(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn grrr(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn grrg(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn grrb(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn grra(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn grgr(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn grgg(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn grgb(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn grga(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn grbr(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn grbg(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn grbb(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn grba(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_grba(&mut self, value: V4<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; self.3 = value.3; }
  pub fn grar(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn grag(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn grab(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_grab(&mut self, value: V4<T>) { self.1 = value.0; self.0 = value.1; self.3 = value.2; self.2 = value.3; }
  pub fn graa(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn ggrr(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn ggrg(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn ggrb(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn ggra(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn gggr(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn gggg(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn gggb(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn ggga(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn ggbr(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn ggbg(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn ggbb(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn ggba(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn ggar(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn ggag(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn ggab(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn ggaa(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn gbrr(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn gbrg(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn gbrb(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn gbra(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_gbra(&mut self, value: V4<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; self.3 = value.3; }
  pub fn gbgr(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn gbgg(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn gbgb(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn gbga(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn gbbr(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn gbbg(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn gbbb(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn gbba(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn gbar(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_gbar(&mut self, value: V4<T>) { self.1 = value.0; self.2 = value.1; self.3 = value.2; self.0 = value.3; }
  pub fn gbag(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn gbab(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn gbaa(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn garr(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn garg(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn garb(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_garb(&mut self, value: V4<T>) { self.1 = value.0; self.3 = value.1; self.0 = value.2; self.2 = value.3; }
  pub fn gara(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn gagr(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn gagg(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn gagb(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn gaga(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn gabr(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_gabr(&mut self, value: V4<T>) { self.1 = value.0; self.3 = value.1; self.2 = value.2; self.0 = value.3; }
  pub fn gabg(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn gabb(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn gaba(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn gaar(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn gaag(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn gaab(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn gaaa(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn brrr(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn brrg(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn brrb(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn brra(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn brgr(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn brgg(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn brgb(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn brga(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_brga(&mut self, value: V4<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; self.3 = value.3; }
  pub fn brbr(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn brbg(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn brbb(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn brba(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn brar(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn brag(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_brag(&mut self, value: V4<T>) { self.2 = value.0; self.0 = value.1; self.3 = value.2; self.1 = value.3; }
  pub fn brab(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn braa(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn bgrr(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn bgrg(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn bgrb(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn bgra(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_bgra(&mut self, value: V4<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; self.3 = value.3; }
  pub fn bggr(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn bggg(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn bggb(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn bgga(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn bgbr(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn bgbg(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn bgbb(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn bgba(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn bgar(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_bgar(&mut self, value: V4<T>) { self.2 = value.0; self.1 = value.1; self.3 = value.2; self.0 = value.3; }
  pub fn bgag(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn bgab(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn bgaa(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn bbrr(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn bbrg(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn bbrb(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn bbra(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn bbgr(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn bbgg(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn bbgb(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn bbga(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn bbbr(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn bbbg(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn bbbb(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn bbba(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn bbar(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn bbag(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn bbab(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn bbaa(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn barr(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn barg(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_barg(&mut self, value: V4<T>) { self.2 = value.0; self.3 = value.1; self.0 = value.2; self.1 = value.3; }
  pub fn barb(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn bara(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn bagr(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_bagr(&mut self, value: V4<T>) { self.2 = value.0; self.3 = value.1; self.1 = value.2; self.0 = value.3; }
  pub fn bagg(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn bagb(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn baga(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn babr(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn babg(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn babb(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn baba(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn baar(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn baag(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn baab(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn baaa(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn arrr(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn arrg(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn arrb(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn arra(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn argr(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn argg(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn argb(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_argb(&mut self, value: V4<T>) { self.3 = value.0; self.0 = value.1; self.1 = value.2; self.2 = value.3; }
  pub fn arga(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn arbr(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn arbg(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_arbg(&mut self, value: V4<T>) { self.3 = value.0; self.0 = value.1; self.2 = value.2; self.1 = value.3; }
  pub fn arbb(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn arba(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn arar(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn arag(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn arab(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn araa(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn agrr(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn agrg(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn agrb(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_agrb(&mut self, value: V4<T>) { self.3 = value.0; self.1 = value.1; self.0 = value.2; self.2 = value.3; }
  pub fn agra(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn aggr(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn aggg(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn aggb(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn agga(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn agbr(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_agbr(&mut self, value: V4<T>) { self.3 = value.0; self.1 = value.1; self.2 = value.2; self.0 = value.3; }
  pub fn agbg(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn agbb(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn agba(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn agar(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn agag(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn agab(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn agaa(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn abrr(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn abrg(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_abrg(&mut self, value: V4<T>) { self.3 = value.0; self.2 = value.1; self.0 = value.2; self.1 = value.3; }
  pub fn abrb(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn abra(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn abgr(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_abgr(&mut self, value: V4<T>) { self.3 = value.0; self.2 = value.1; self.1 = value.2; self.0 = value.3; }
  pub fn abgg(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn abgb(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn abga(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn abbr(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn abbg(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn abbb(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn abba(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn abar(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn abag(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn abab(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn abaa(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn aarr(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn aarg(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn aarb(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn aara(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn aagr(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn aagg(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn aagb(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn aaga(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn aabr(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn aabg(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn aabb(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn aaba(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn aaar(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn aaag(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn aaab(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn aaaa(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn s(&self) -> T { self.0.clone() }
  pub fn set_s(&mut self, value: T) { self.0 = value; }
  pub fn t(&self) -> T { self.1.clone() }
  pub fn set_t(&mut self, value: T) { self.1 = value; }
  pub fn p(&self) -> T { self.2.clone() }
  pub fn set_p(&mut self, value: T) { self.2 = value; }
  pub fn q(&self) -> T { self.3.clone() }
  pub fn set_q(&mut self, value: T) { self.3 = value; }
  pub fn ss(&self) -> V2<T> { V2(self.0.clone(), self.0.clone()) }
  pub fn st(&self) -> V2<T> { V2(self.0.clone(), self.1.clone()) }
  pub fn set_st(&mut self, value: V2<T>) { self.0 = value.0; self.1 = value.1; }
  pub fn sp(&self) -> V2<T> { V2(self.0.clone(), self.2.clone()) }
  pub fn set_sp(&mut self, value: V2<T>) { self.0 = value.0; self.2 = value.1; }
  pub fn sq(&self) -> V2<T> { V2(self.0.clone(), self.3.clone()) }
  pub fn set_sq(&mut self, value: V2<T>) { self.0 = value.0; self.3 = value.1; }
  pub fn ts(&self) -> V2<T> { V2(self.1.clone(), self.0.clone()) }
  pub fn set_ts(&mut self, value: V2<T>) { self.1 = value.0; self.0 = value.1; }
  pub fn tt(&self) -> V2<T> { V2(self.1.clone(), self.1.clone()) }
  pub fn tp(&self) -> V2<T> { V2(self.1.clone(), self.2.clone()) }
  pub fn set_tp(&mut self, value: V2<T>) { self.1 = value.0; self.2 = value.1; }
  pub fn tq(&self) -> V2<T> { V2(self.1.clone(), self.3.clone()) }
  pub fn set_tq(&mut self, value: V2<T>) { self.1 = value.0; self.3 = value.1; }
  pub fn ps(&self) -> V2<T> { V2(self.2.clone(), self.0.clone()) }
  pub fn set_ps(&mut self, value: V2<T>) { self.2 = value.0; self.0 = value.1; }
  pub fn pt(&self) -> V2<T> { V2(self.2.clone(), self.1.clone()) }
  pub fn set_pt(&mut self, value: V2<T>) { self.2 = value.0; self.1 = value.1; }
  pub fn pp(&self) -> V2<T> { V2(self.2.clone(), self.2.clone()) }
  pub fn pq(&self) -> V2<T> { V2(self.2.clone(), self.3.clone()) }
  pub fn set_pq(&mut self, value: V2<T>) { self.2 = value.0; self.3 = value.1; }
  pub fn qs(&self) -> V2<T> { V2(self.3.clone(), self.0.clone()) }
  pub fn set_qs(&mut self, value: V2<T>) { self.3 = value.0; self.0 = value.1; }
  pub fn qt(&self) -> V2<T> { V2(self.3.clone(), self.1.clone()) }
  pub fn set_qt(&mut self, value: V2<T>) { self.3 = value.0; self.1 = value.1; }
  pub fn qp(&self) -> V2<T> { V2(self.3.clone(), self.2.clone()) }
  pub fn set_qp(&mut self, value: V2<T>) { self.3 = value.0; self.2 = value.1; }
  pub fn qq(&self) -> V2<T> { V2(self.3.clone(), self.3.clone()) }
  pub fn sss(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn sst(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn ssp(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn ssq(&self) -> V3<T> { V3(self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn sts(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn stt(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn stp(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_stp(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn stq(&self) -> V3<T> { V3(self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_stq(&mut self, value: V3<T>) { self.0 = value.0; self.1 = value.1; self.3 = value.2; }
  pub fn sps(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn spt(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_spt(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn spp(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn spq(&self) -> V3<T> { V3(self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_spq(&mut self, value: V3<T>) { self.0 = value.0; self.2 = value.1; self.3 = value.2; }
  pub fn sqs(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn sqt(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_sqt(&mut self, value: V3<T>) { self.0 = value.0; self.3 = value.1; self.1 = value.2; }
  pub fn sqp(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_sqp(&mut self, value: V3<T>) { self.0 = value.0; self.3 = value.1; self.2 = value.2; }
  pub fn sqq(&self) -> V3<T> { V3(self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn tss(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn tst(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn tsp(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_tsp(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn tsq(&self) -> V3<T> { V3(self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_tsq(&mut self, value: V3<T>) { self.1 = value.0; self.0 = value.1; self.3 = value.2; }
  pub fn tts(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn ttt(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn ttp(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn ttq(&self) -> V3<T> { V3(self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn tps(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_tps(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn tpt(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn tpp(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn tpq(&self) -> V3<T> { V3(self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_tpq(&mut self, value: V3<T>) { self.1 = value.0; self.2 = value.1; self.3 = value.2; }
  pub fn tqs(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_tqs(&mut self, value: V3<T>) { self.1 = value.0; self.3 = value.1; self.0 = value.2; }
  pub fn tqt(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn tqp(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_tqp(&mut self, value: V3<T>) { self.1 = value.0; self.3 = value.1; self.2 = value.2; }
  pub fn tqq(&self) -> V3<T> { V3(self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn pss(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn pst(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_pst(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn psp(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn psq(&self) -> V3<T> { V3(self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_psq(&mut self, value: V3<T>) { self.2 = value.0; self.0 = value.1; self.3 = value.2; }
  pub fn pts(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_pts(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn ptt(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn ptp(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn ptq(&self) -> V3<T> { V3(self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_ptq(&mut self, value: V3<T>) { self.2 = value.0; self.1 = value.1; self.3 = value.2; }
  pub fn pps(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn ppt(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn ppp(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn ppq(&self) -> V3<T> { V3(self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn pqs(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_pqs(&mut self, value: V3<T>) { self.2 = value.0; self.3 = value.1; self.0 = value.2; }
  pub fn pqt(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_pqt(&mut self, value: V3<T>) { self.2 = value.0; self.3 = value.1; self.1 = value.2; }
  pub fn pqp(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn pqq(&self) -> V3<T> { V3(self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn qss(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn qst(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_qst(&mut self, value: V3<T>) { self.3 = value.0; self.0 = value.1; self.1 = value.2; }
  pub fn qsp(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_qsp(&mut self, value: V3<T>) { self.3 = value.0; self.0 = value.1; self.2 = value.2; }
  pub fn qsq(&self) -> V3<T> { V3(self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn qts(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_qts(&mut self, value: V3<T>) { self.3 = value.0; self.1 = value.1; self.0 = value.2; }
  pub fn qtt(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn qtp(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_qtp(&mut self, value: V3<T>) { self.3 = value.0; self.1 = value.1; self.2 = value.2; }
  pub fn qtq(&self) -> V3<T> { V3(self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn qps(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_qps(&mut self, value: V3<T>) { self.3 = value.0; self.2 = value.1; self.0 = value.2; }
  pub fn qpt(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_qpt(&mut self, value: V3<T>) { self.3 = value.0; self.2 = value.1; self.1 = value.2; }
  pub fn qpp(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn qpq(&self) -> V3<T> { V3(self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn qqs(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn qqt(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn qqp(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn qqq(&self) -> V3<T> { V3(self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn ssss(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn ssst(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn sssp(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn sssq(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn ssts(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn sstt(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn sstp(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn sstq(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn ssps(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn sspt(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn sspp(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn sspq(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn ssqs(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn ssqt(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn ssqp(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn ssqq(&self) -> V4<T> { V4(self.0.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn stss(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn stst(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn stsp(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn stsq(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn stts(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn sttt(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn sttp(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn sttq(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn stps(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn stpt(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn stpp(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn stpq(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_stpq(&mut self, value: V4<T>) { self.0 = value.0; self.1 = value.1; self.2 = value.2; self.3 = value.3; }
  pub fn stqs(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn stqt(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn stqp(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_stqp(&mut self, value: V4<T>) { self.0 = value.0; self.1 = value.1; self.3 = value.2; self.2 = value.3; }
  pub fn stqq(&self) -> V4<T> { V4(self.0.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn spss(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn spst(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn spsp(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn spsq(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn spts(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn sptt(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn sptp(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn sptq(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_sptq(&mut self, value: V4<T>) { self.0 = value.0; self.2 = value.1; self.1 = value.2; self.3 = value.3; }
  pub fn spps(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn sppt(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn sppp(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn sppq(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn spqs(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn spqt(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_spqt(&mut self, value: V4<T>) { self.0 = value.0; self.2 = value.1; self.3 = value.2; self.1 = value.3; }
  pub fn spqp(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn spqq(&self) -> V4<T> { V4(self.0.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn sqss(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn sqst(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn sqsp(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn sqsq(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn sqts(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn sqtt(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn sqtp(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_sqtp(&mut self, value: V4<T>) { self.0 = value.0; self.3 = value.1; self.1 = value.2; self.2 = value.3; }
  pub fn sqtq(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn sqps(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn sqpt(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_sqpt(&mut self, value: V4<T>) { self.0 = value.0; self.3 = value.1; self.2 = value.2; self.1 = value.3; }
  pub fn sqpp(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn sqpq(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn sqqs(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn sqqt(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn sqqp(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn sqqq(&self) -> V4<T> { V4(self.0.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn tsss(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn tsst(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn tssp(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn tssq(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn tsts(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn tstt(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn tstp(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn tstq(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn tsps(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn tspt(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn tspp(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn tspq(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn set_tspq(&mut self, value: V4<T>) { self.1 = value.0; self.0 = value.1; self.2 = value.2; self.3 = value.3; }
  pub fn tsqs(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn tsqt(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn tsqp(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn set_tsqp(&mut self, value: V4<T>) { self.1 = value.0; self.0 = value.1; self.3 = value.2; self.2 = value.3; }
  pub fn tsqq(&self) -> V4<T> { V4(self.1.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn ttss(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn ttst(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn ttsp(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn ttsq(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn ttts(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn tttt(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn tttp(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn tttq(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn ttps(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn ttpt(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn ttpp(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn ttpq(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn ttqs(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn ttqt(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn ttqp(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn ttqq(&self) -> V4<T> { V4(self.1.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn tpss(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn tpst(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn tpsp(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn tpsq(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_tpsq(&mut self, value: V4<T>) { self.1 = value.0; self.2 = value.1; self.0 = value.2; self.3 = value.3; }
  pub fn tpts(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn tptt(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn tptp(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn tptq(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn tpps(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn tppt(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn tppp(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn tppq(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn tpqs(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_tpqs(&mut self, value: V4<T>) { self.1 = value.0; self.2 = value.1; self.3 = value.2; self.0 = value.3; }
  pub fn tpqt(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn tpqp(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn tpqq(&self) -> V4<T> { V4(self.1.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn tqss(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn tqst(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn tqsp(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_tqsp(&mut self, value: V4<T>) { self.1 = value.0; self.3 = value.1; self.0 = value.2; self.2 = value.3; }
  pub fn tqsq(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn tqts(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn tqtt(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn tqtp(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn tqtq(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn tqps(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_tqps(&mut self, value: V4<T>) { self.1 = value.0; self.3 = value.1; self.2 = value.2; self.0 = value.3; }
  pub fn tqpt(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn tqpp(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn tqpq(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn tqqs(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn tqqt(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn tqqp(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn tqqq(&self) -> V4<T> { V4(self.1.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn psss(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn psst(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn pssp(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn pssq(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn psts(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn pstt(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn pstp(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn pstq(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn set_pstq(&mut self, value: V4<T>) { self.2 = value.0; self.0 = value.1; self.1 = value.2; self.3 = value.3; }
  pub fn psps(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn pspt(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn pspp(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn pspq(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn psqs(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn psqt(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn set_psqt(&mut self, value: V4<T>) { self.2 = value.0; self.0 = value.1; self.3 = value.2; self.1 = value.3; }
  pub fn psqp(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn psqq(&self) -> V4<T> { V4(self.2.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn ptss(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn ptst(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn ptsp(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn ptsq(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn set_ptsq(&mut self, value: V4<T>) { self.2 = value.0; self.1 = value.1; self.0 = value.2; self.3 = value.3; }
  pub fn ptts(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn pttt(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn pttp(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn pttq(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn ptps(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn ptpt(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn ptpp(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn ptpq(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn ptqs(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn set_ptqs(&mut self, value: V4<T>) { self.2 = value.0; self.1 = value.1; self.3 = value.2; self.0 = value.3; }
  pub fn ptqt(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn ptqp(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn ptqq(&self) -> V4<T> { V4(self.2.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn ppss(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn ppst(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn ppsp(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn ppsq(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn ppts(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn pptt(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn pptp(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn pptq(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn ppps(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn pppt(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn pppp(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn pppq(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn ppqs(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn ppqt(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn ppqp(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn ppqq(&self) -> V4<T> { V4(self.2.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn pqss(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn pqst(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_pqst(&mut self, value: V4<T>) { self.2 = value.0; self.3 = value.1; self.0 = value.2; self.1 = value.3; }
  pub fn pqsp(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn pqsq(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn pqts(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_pqts(&mut self, value: V4<T>) { self.2 = value.0; self.3 = value.1; self.1 = value.2; self.0 = value.3; }
  pub fn pqtt(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn pqtp(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn pqtq(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn pqps(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn pqpt(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn pqpp(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn pqpq(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn pqqs(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn pqqt(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn pqqp(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn pqqq(&self) -> V4<T> { V4(self.2.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
  pub fn qsss(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.0.clone()) }
  pub fn qsst(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.1.clone()) }
  pub fn qssp(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.2.clone()) }
  pub fn qssq(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.0.clone(), self.3.clone()) }
  pub fn qsts(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.0.clone()) }
  pub fn qstt(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.1.clone()) }
  pub fn qstp(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.2.clone()) }
  pub fn set_qstp(&mut self, value: V4<T>) { self.3 = value.0; self.0 = value.1; self.1 = value.2; self.2 = value.3; }
  pub fn qstq(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.1.clone(), self.3.clone()) }
  pub fn qsps(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.0.clone()) }
  pub fn qspt(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.1.clone()) }
  pub fn set_qspt(&mut self, value: V4<T>) { self.3 = value.0; self.0 = value.1; self.2 = value.2; self.1 = value.3; }
  pub fn qspp(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.2.clone()) }
  pub fn qspq(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.2.clone(), self.3.clone()) }
  pub fn qsqs(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.0.clone()) }
  pub fn qsqt(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.1.clone()) }
  pub fn qsqp(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.2.clone()) }
  pub fn qsqq(&self) -> V4<T> { V4(self.3.clone(), self.0.clone(), self.3.clone(), self.3.clone()) }
  pub fn qtss(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.0.clone()) }
  pub fn qtst(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.1.clone()) }
  pub fn qtsp(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.2.clone()) }
  pub fn set_qtsp(&mut self, value: V4<T>) { self.3 = value.0; self.1 = value.1; self.0 = value.2; self.2 = value.3; }
  pub fn qtsq(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.0.clone(), self.3.clone()) }
  pub fn qtts(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.0.clone()) }
  pub fn qttt(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.1.clone()) }
  pub fn qttp(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.2.clone()) }
  pub fn qttq(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.1.clone(), self.3.clone()) }
  pub fn qtps(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.0.clone()) }
  pub fn set_qtps(&mut self, value: V4<T>) { self.3 = value.0; self.1 = value.1; self.2 = value.2; self.0 = value.3; }
  pub fn qtpt(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.1.clone()) }
  pub fn qtpp(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.2.clone()) }
  pub fn qtpq(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.2.clone(), self.3.clone()) }
  pub fn qtqs(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.0.clone()) }
  pub fn qtqt(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.1.clone()) }
  pub fn qtqp(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.2.clone()) }
  pub fn qtqq(&self) -> V4<T> { V4(self.3.clone(), self.1.clone(), self.3.clone(), self.3.clone()) }
  pub fn qpss(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.0.clone()) }
  pub fn qpst(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.1.clone()) }
  pub fn set_qpst(&mut self, value: V4<T>) { self.3 = value.0; self.2 = value.1; self.0 = value.2; self.1 = value.3; }
  pub fn qpsp(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.2.clone()) }
  pub fn qpsq(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.0.clone(), self.3.clone()) }
  pub fn qpts(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.0.clone()) }
  pub fn set_qpts(&mut self, value: V4<T>) { self.3 = value.0; self.2 = value.1; self.1 = value.2; self.0 = value.3; }
  pub fn qptt(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.1.clone()) }
  pub fn qptp(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.2.clone()) }
  pub fn qptq(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.1.clone(), self.3.clone()) }
  pub fn qpps(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.0.clone()) }
  pub fn qppt(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.1.clone()) }
  pub fn qppp(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.2.clone()) }
  pub fn qppq(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.2.clone(), self.3.clone()) }
  pub fn qpqs(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.0.clone()) }
  pub fn qpqt(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.1.clone()) }
  pub fn qpqp(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.2.clone()) }
  pub fn qpqq(&self) -> V4<T> { V4(self.3.clone(), self.2.clone(), self.3.clone(), self.3.clone()) }
  pub fn qqss(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.0.clone()) }
  pub fn qqst(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.1.clone()) }
  pub fn qqsp(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.2.clone()) }
  pub fn qqsq(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.0.clone(), self.3.clone()) }
  pub fn qqts(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.0.clone()) }
  pub fn qqtt(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.1.clone()) }
  pub fn qqtp(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.2.clone()) }
  pub fn qqtq(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.1.clone(), self.3.clone()) }
  pub fn qqps(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.0.clone()) }
  pub fn qqpt(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.1.clone()) }
  pub fn qqpp(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.2.clone()) }
  pub fn qqpq(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.2.clone(), self.3.clone()) }
  pub fn qqqs(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.0.clone()) }
  pub fn qqqt(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.1.clone()) }
  pub fn qqqp(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.2.clone()) }
  pub fn qqqq(&self) -> V4<T> { V4(self.3.clone(), self.3.clone(), self.3.clone(), self.3.clone()) }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn swizzle2_test() {
    let mut v2 = V2(0, 0);
    v2.set_x(1);
    assert_eq!(v2.x(), 1);
    v2.set_y(2);
    assert_eq!(v2.y(), 2);
    assert_eq!(v2.xx(), V2(1, 1));
    v2.set_xy(V2(3, 4));
    assert_eq!(v2.xy(), V2(3, 4));
    v2.set_yx(V2(5, 6));
    assert_eq!(v2.yx(), V2(5, 6));
    assert_eq!(v2.yy(), V2(5, 5));
    v2.set_r(7);
    assert_eq!(v2.r(), 7);
    v2.set_g(8);
    assert_eq!(v2.g(), 8);
    assert_eq!(v2.rr(), V2(7, 7));
    v2.set_rg(V2(9, 10));
    assert_eq!(v2.rg(), V2(9, 10));
    v2.set_gr(V2(11, 12));
    assert_eq!(v2.gr(), V2(11, 12));
    assert_eq!(v2.gg(), V2(11, 11));
    v2.set_s(13);
    assert_eq!(v2.s(), 13);
    v2.set_t(14);
    assert_eq!(v2.t(), 14);
    assert_eq!(v2.ss(), V2(13, 13));
    v2.set_st(V2(15, 16));
    assert_eq!(v2.st(), V2(15, 16));
    v2.set_ts(V2(17, 18));
    assert_eq!(v2.ts(), V2(17, 18));
    assert_eq!(v2.tt(), V2(17, 17));
  }

  #[test]
  fn swizzle3_test() {
    let mut v3 = V3(0, 0, 0);
    v3.set_x(1);
    assert_eq!(v3.x(), 1);
    v3.set_y(2);
    assert_eq!(v3.y(), 2);
    v3.set_z(3);
    assert_eq!(v3.z(), 3);
    assert_eq!(v3.xx(), V2(1, 1));
    v3.set_xy(V2(4, 5));
    assert_eq!(v3.xy(), V2(4, 5));
    v3.set_xz(V2(6, 7));
    assert_eq!(v3.xz(), V2(6, 7));
    v3.set_yx(V2(8, 9));
    assert_eq!(v3.yx(), V2(8, 9));
    assert_eq!(v3.yy(), V2(8, 8));
    v3.set_yz(V2(10, 11));
    assert_eq!(v3.yz(), V2(10, 11));
    v3.set_zx(V2(12, 13));
    assert_eq!(v3.zx(), V2(12, 13));
    v3.set_zy(V2(14, 15));
    assert_eq!(v3.zy(), V2(14, 15));
    assert_eq!(v3.zz(), V2(14, 14));
    assert_eq!(v3.xxx(), V3(13, 13, 13));
    assert_eq!(v3.xxy(), V3(13, 13, 15));
    assert_eq!(v3.xxz(), V3(13, 13, 14));
    assert_eq!(v3.xyx(), V3(13, 15, 13));
    assert_eq!(v3.xyy(), V3(13, 15, 15));
    v3.set_xyz(V3(16, 17, 18));
    assert_eq!(v3.xyz(), V3(16, 17, 18));
    assert_eq!(v3.xzx(), V3(16, 18, 16));
    v3.set_xzy(V3(19, 20, 21));
    assert_eq!(v3.xzy(), V3(19, 20, 21));
    assert_eq!(v3.xzz(), V3(19, 20, 20));
    assert_eq!(v3.yxx(), V3(21, 19, 19));
    assert_eq!(v3.yxy(), V3(21, 19, 21));
    v3.set_yxz(V3(22, 23, 24));
    assert_eq!(v3.yxz(), V3(22, 23, 24));
    assert_eq!(v3.yyx(), V3(22, 22, 23));
    assert_eq!(v3.yyy(), V3(22, 22, 22));
    assert_eq!(v3.yyz(), V3(22, 22, 24));
    v3.set_yzx(V3(25, 26, 27));
    assert_eq!(v3.yzx(), V3(25, 26, 27));
    assert_eq!(v3.yzy(), V3(25, 26, 25));
    assert_eq!(v3.yzz(), V3(25, 26, 26));
    assert_eq!(v3.zxx(), V3(26, 27, 27));
    v3.set_zxy(V3(28, 29, 30));
    assert_eq!(v3.zxy(), V3(28, 29, 30));
    assert_eq!(v3.zxz(), V3(28, 29, 28));
    v3.set_zyx(V3(31, 32, 33));
    assert_eq!(v3.zyx(), V3(31, 32, 33));
    assert_eq!(v3.zyy(), V3(31, 32, 32));
    assert_eq!(v3.zyz(), V3(31, 32, 31));
    assert_eq!(v3.zzx(), V3(31, 31, 33));
    assert_eq!(v3.zzy(), V3(31, 31, 32));
    assert_eq!(v3.zzz(), V3(31, 31, 31));
    v3.set_r(34);
    assert_eq!(v3.r(), 34);
    v3.set_g(35);
    assert_eq!(v3.g(), 35);
    v3.set_b(36);
    assert_eq!(v3.b(), 36);
    assert_eq!(v3.rr(), V2(34, 34));
    v3.set_rg(V2(37, 38));
    assert_eq!(v3.rg(), V2(37, 38));
    v3.set_rb(V2(39, 40));
    assert_eq!(v3.rb(), V2(39, 40));
    v3.set_gr(V2(41, 42));
    assert_eq!(v3.gr(), V2(41, 42));
    assert_eq!(v3.gg(), V2(41, 41));
    v3.set_gb(V2(43, 44));
    assert_eq!(v3.gb(), V2(43, 44));
    v3.set_br(V2(45, 46));
    assert_eq!(v3.br(), V2(45, 46));
    v3.set_bg(V2(47, 48));
    assert_eq!(v3.bg(), V2(47, 48));
    assert_eq!(v3.bb(), V2(47, 47));
    assert_eq!(v3.rrr(), V3(46, 46, 46));
    assert_eq!(v3.rrg(), V3(46, 46, 48));
    assert_eq!(v3.rrb(), V3(46, 46, 47));
    assert_eq!(v3.rgr(), V3(46, 48, 46));
    assert_eq!(v3.rgg(), V3(46, 48, 48));
    v3.set_rgb(V3(49, 50, 51));
    assert_eq!(v3.rgb(), V3(49, 50, 51));
    assert_eq!(v3.rbr(), V3(49, 51, 49));
    v3.set_rbg(V3(52, 53, 54));
    assert_eq!(v3.rbg(), V3(52, 53, 54));
    assert_eq!(v3.rbb(), V3(52, 53, 53));
    assert_eq!(v3.grr(), V3(54, 52, 52));
    assert_eq!(v3.grg(), V3(54, 52, 54));
    v3.set_grb(V3(55, 56, 57));
    assert_eq!(v3.grb(), V3(55, 56, 57));
    assert_eq!(v3.ggr(), V3(55, 55, 56));
    assert_eq!(v3.ggg(), V3(55, 55, 55));
    assert_eq!(v3.ggb(), V3(55, 55, 57));
    v3.set_gbr(V3(58, 59, 60));
    assert_eq!(v3.gbr(), V3(58, 59, 60));
    assert_eq!(v3.gbg(), V3(58, 59, 58));
    assert_eq!(v3.gbb(), V3(58, 59, 59));
    assert_eq!(v3.brr(), V3(59, 60, 60));
    v3.set_brg(V3(61, 62, 63));
    assert_eq!(v3.brg(), V3(61, 62, 63));
    assert_eq!(v3.brb(), V3(61, 62, 61));
    v3.set_bgr(V3(64, 65, 66));
    assert_eq!(v3.bgr(), V3(64, 65, 66));
    assert_eq!(v3.bgg(), V3(64, 65, 65));
    assert_eq!(v3.bgb(), V3(64, 65, 64));
    assert_eq!(v3.bbr(), V3(64, 64, 66));
    assert_eq!(v3.bbg(), V3(64, 64, 65));
    assert_eq!(v3.bbb(), V3(64, 64, 64));
    v3.set_s(67);
    assert_eq!(v3.s(), 67);
    v3.set_t(68);
    assert_eq!(v3.t(), 68);
    v3.set_p(69);
    assert_eq!(v3.p(), 69);
    assert_eq!(v3.ss(), V2(67, 67));
    v3.set_st(V2(70, 71));
    assert_eq!(v3.st(), V2(70, 71));
    v3.set_sp(V2(72, 73));
    assert_eq!(v3.sp(), V2(72, 73));
    v3.set_ts(V2(74, 75));
    assert_eq!(v3.ts(), V2(74, 75));
    assert_eq!(v3.tt(), V2(74, 74));
    v3.set_tp(V2(76, 77));
    assert_eq!(v3.tp(), V2(76, 77));
    v3.set_ps(V2(78, 79));
    assert_eq!(v3.ps(), V2(78, 79));
    v3.set_pt(V2(80, 81));
    assert_eq!(v3.pt(), V2(80, 81));
    assert_eq!(v3.pp(), V2(80, 80));
    assert_eq!(v3.sss(), V3(79, 79, 79));
    assert_eq!(v3.sst(), V3(79, 79, 81));
    assert_eq!(v3.ssp(), V3(79, 79, 80));
    assert_eq!(v3.sts(), V3(79, 81, 79));
    assert_eq!(v3.stt(), V3(79, 81, 81));
    v3.set_stp(V3(82, 83, 84));
    assert_eq!(v3.stp(), V3(82, 83, 84));
    assert_eq!(v3.sps(), V3(82, 84, 82));
    v3.set_spt(V3(85, 86, 87));
    assert_eq!(v3.spt(), V3(85, 86, 87));
    assert_eq!(v3.spp(), V3(85, 86, 86));
    assert_eq!(v3.tss(), V3(87, 85, 85));
    assert_eq!(v3.tst(), V3(87, 85, 87));
    v3.set_tsp(V3(88, 89, 90));
    assert_eq!(v3.tsp(), V3(88, 89, 90));
    assert_eq!(v3.tts(), V3(88, 88, 89));
    assert_eq!(v3.ttt(), V3(88, 88, 88));
    assert_eq!(v3.ttp(), V3(88, 88, 90));
    v3.set_tps(V3(91, 92, 93));
    assert_eq!(v3.tps(), V3(91, 92, 93));
    assert_eq!(v3.tpt(), V3(91, 92, 91));
    assert_eq!(v3.tpp(), V3(91, 92, 92));
    assert_eq!(v3.pss(), V3(92, 93, 93));
    v3.set_pst(V3(94, 95, 96));
    assert_eq!(v3.pst(), V3(94, 95, 96));
    assert_eq!(v3.psp(), V3(94, 95, 94));
    v3.set_pts(V3(97, 98, 99));
    assert_eq!(v3.pts(), V3(97, 98, 99));
    assert_eq!(v3.ptt(), V3(97, 98, 98));
    assert_eq!(v3.ptp(), V3(97, 98, 97));
    assert_eq!(v3.pps(), V3(97, 97, 99));
    assert_eq!(v3.ppt(), V3(97, 97, 98));
    assert_eq!(v3.ppp(), V3(97, 97, 97));
  }

  #[test]
  fn swizzle4_test() {
    let mut v4 = V4(0, 0, 0, 0);
    v4.set_x(1);
    assert_eq!(v4.x(), 1);
    v4.set_y(2);
    assert_eq!(v4.y(), 2);
    v4.set_z(3);
    assert_eq!(v4.z(), 3);
    v4.set_w(4);
    assert_eq!(v4.w(), 4);
    assert_eq!(v4.xx(), V2(1, 1));
    v4.set_xy(V2(5, 6));
    assert_eq!(v4.xy(), V2(5, 6));
    v4.set_xz(V2(7, 8));
    assert_eq!(v4.xz(), V2(7, 8));
    v4.set_xw(V2(9, 10));
    assert_eq!(v4.xw(), V2(9, 10));
    v4.set_yx(V2(11, 12));
    assert_eq!(v4.yx(), V2(11, 12));
    assert_eq!(v4.yy(), V2(11, 11));
    v4.set_yz(V2(13, 14));
    assert_eq!(v4.yz(), V2(13, 14));
    v4.set_yw(V2(15, 16));
    assert_eq!(v4.yw(), V2(15, 16));
    v4.set_zx(V2(17, 18));
    assert_eq!(v4.zx(), V2(17, 18));
    v4.set_zy(V2(19, 20));
    assert_eq!(v4.zy(), V2(19, 20));
    assert_eq!(v4.zz(), V2(19, 19));
    v4.set_zw(V2(21, 22));
    assert_eq!(v4.zw(), V2(21, 22));
    v4.set_wx(V2(23, 24));
    assert_eq!(v4.wx(), V2(23, 24));
    v4.set_wy(V2(25, 26));
    assert_eq!(v4.wy(), V2(25, 26));
    v4.set_wz(V2(27, 28));
    assert_eq!(v4.wz(), V2(27, 28));
    assert_eq!(v4.ww(), V2(27, 27));
    assert_eq!(v4.xxx(), V3(24, 24, 24));
    assert_eq!(v4.xxy(), V3(24, 24, 26));
    assert_eq!(v4.xxz(), V3(24, 24, 28));
    assert_eq!(v4.xxw(), V3(24, 24, 27));
    assert_eq!(v4.xyx(), V3(24, 26, 24));
    assert_eq!(v4.xyy(), V3(24, 26, 26));
    v4.set_xyz(V3(29, 30, 31));
    assert_eq!(v4.xyz(), V3(29, 30, 31));
    v4.set_xyw(V3(32, 33, 34));
    assert_eq!(v4.xyw(), V3(32, 33, 34));
    assert_eq!(v4.xzx(), V3(32, 31, 32));
    v4.set_xzy(V3(35, 36, 37));
    assert_eq!(v4.xzy(), V3(35, 36, 37));
    assert_eq!(v4.xzz(), V3(35, 36, 36));
    v4.set_xzw(V3(38, 39, 40));
    assert_eq!(v4.xzw(), V3(38, 39, 40));
    assert_eq!(v4.xwx(), V3(38, 40, 38));
    v4.set_xwy(V3(41, 42, 43));
    assert_eq!(v4.xwy(), V3(41, 42, 43));
    v4.set_xwz(V3(44, 45, 46));
    assert_eq!(v4.xwz(), V3(44, 45, 46));
    assert_eq!(v4.xww(), V3(44, 45, 45));
    assert_eq!(v4.yxx(), V3(43, 44, 44));
    assert_eq!(v4.yxy(), V3(43, 44, 43));
    v4.set_yxz(V3(47, 48, 49));
    assert_eq!(v4.yxz(), V3(47, 48, 49));
    v4.set_yxw(V3(50, 51, 52));
    assert_eq!(v4.yxw(), V3(50, 51, 52));
    assert_eq!(v4.yyx(), V3(50, 50, 51));
    assert_eq!(v4.yyy(), V3(50, 50, 50));
    assert_eq!(v4.yyz(), V3(50, 50, 49));
    assert_eq!(v4.yyw(), V3(50, 50, 52));
    v4.set_yzx(V3(53, 54, 55));
    assert_eq!(v4.yzx(), V3(53, 54, 55));
    assert_eq!(v4.yzy(), V3(53, 54, 53));
    assert_eq!(v4.yzz(), V3(53, 54, 54));
    v4.set_yzw(V3(56, 57, 58));
    assert_eq!(v4.yzw(), V3(56, 57, 58));
    v4.set_ywx(V3(59, 60, 61));
    assert_eq!(v4.ywx(), V3(59, 60, 61));
    assert_eq!(v4.ywy(), V3(59, 60, 59));
    v4.set_ywz(V3(62, 63, 64));
    assert_eq!(v4.ywz(), V3(62, 63, 64));
    assert_eq!(v4.yww(), V3(62, 63, 63));
    assert_eq!(v4.zxx(), V3(64, 61, 61));
    v4.set_zxy(V3(65, 66, 67));
    assert_eq!(v4.zxy(), V3(65, 66, 67));
    assert_eq!(v4.zxz(), V3(65, 66, 65));
    v4.set_zxw(V3(68, 69, 70));
    assert_eq!(v4.zxw(), V3(68, 69, 70));
    v4.set_zyx(V3(71, 72, 73));
    assert_eq!(v4.zyx(), V3(71, 72, 73));
    assert_eq!(v4.zyy(), V3(71, 72, 72));
    assert_eq!(v4.zyz(), V3(71, 72, 71));
    v4.set_zyw(V3(74, 75, 76));
    assert_eq!(v4.zyw(), V3(74, 75, 76));
    assert_eq!(v4.zzx(), V3(74, 74, 73));
    assert_eq!(v4.zzy(), V3(74, 74, 75));
    assert_eq!(v4.zzz(), V3(74, 74, 74));
    assert_eq!(v4.zzw(), V3(74, 74, 76));
    v4.set_zwx(V3(77, 78, 79));
    assert_eq!(v4.zwx(), V3(77, 78, 79));
    v4.set_zwy(V3(80, 81, 82));
    assert_eq!(v4.zwy(), V3(80, 81, 82));
    assert_eq!(v4.zwz(), V3(80, 81, 80));
    assert_eq!(v4.zww(), V3(80, 81, 81));
    assert_eq!(v4.wxx(), V3(81, 79, 79));
    v4.set_wxy(V3(83, 84, 85));
    assert_eq!(v4.wxy(), V3(83, 84, 85));
    v4.set_wxz(V3(86, 87, 88));
    assert_eq!(v4.wxz(), V3(86, 87, 88));
    assert_eq!(v4.wxw(), V3(86, 87, 86));
    v4.set_wyx(V3(89, 90, 91));
    assert_eq!(v4.wyx(), V3(89, 90, 91));
    assert_eq!(v4.wyy(), V3(89, 90, 90));
    v4.set_wyz(V3(92, 93, 94));
    assert_eq!(v4.wyz(), V3(92, 93, 94));
    assert_eq!(v4.wyw(), V3(92, 93, 92));
    v4.set_wzx(V3(95, 96, 97));
    assert_eq!(v4.wzx(), V3(95, 96, 97));
    v4.set_wzy(V3(98, 99, 100));
    assert_eq!(v4.wzy(), V3(98, 99, 100));
    assert_eq!(v4.wzz(), V3(98, 99, 99));
    assert_eq!(v4.wzw(), V3(98, 99, 98));
    assert_eq!(v4.wwx(), V3(98, 98, 97));
    assert_eq!(v4.wwy(), V3(98, 98, 100));
    assert_eq!(v4.wwz(), V3(98, 98, 99));
    assert_eq!(v4.www(), V3(98, 98, 98));
    assert_eq!(v4.xxxx(), V4(97, 97, 97, 97));
    assert_eq!(v4.xxxy(), V4(97, 97, 97, 100));
    assert_eq!(v4.xxxz(), V4(97, 97, 97, 99));
    assert_eq!(v4.xxxw(), V4(97, 97, 97, 98));
    assert_eq!(v4.xxyx(), V4(97, 97, 100, 97));
    assert_eq!(v4.xxyy(), V4(97, 97, 100, 100));
    assert_eq!(v4.xxyz(), V4(97, 97, 100, 99));
    assert_eq!(v4.xxyw(), V4(97, 97, 100, 98));
    assert_eq!(v4.xxzx(), V4(97, 97, 99, 97));
    assert_eq!(v4.xxzy(), V4(97, 97, 99, 100));
    assert_eq!(v4.xxzz(), V4(97, 97, 99, 99));
    assert_eq!(v4.xxzw(), V4(97, 97, 99, 98));
    assert_eq!(v4.xxwx(), V4(97, 97, 98, 97));
    assert_eq!(v4.xxwy(), V4(97, 97, 98, 100));
    assert_eq!(v4.xxwz(), V4(97, 97, 98, 99));
    assert_eq!(v4.xxww(), V4(97, 97, 98, 98));
    assert_eq!(v4.xyxx(), V4(97, 100, 97, 97));
    assert_eq!(v4.xyxy(), V4(97, 100, 97, 100));
    assert_eq!(v4.xyxz(), V4(97, 100, 97, 99));
    assert_eq!(v4.xyxw(), V4(97, 100, 97, 98));
    assert_eq!(v4.xyyx(), V4(97, 100, 100, 97));
    assert_eq!(v4.xyyy(), V4(97, 100, 100, 100));
    assert_eq!(v4.xyyz(), V4(97, 100, 100, 99));
    assert_eq!(v4.xyyw(), V4(97, 100, 100, 98));
    assert_eq!(v4.xyzx(), V4(97, 100, 99, 97));
    assert_eq!(v4.xyzy(), V4(97, 100, 99, 100));
    assert_eq!(v4.xyzz(), V4(97, 100, 99, 99));
    v4.set_xyzw(V4(101, 102, 103, 104));
    assert_eq!(v4.xyzw(), V4(101, 102, 103, 104));
    assert_eq!(v4.xywx(), V4(101, 102, 104, 101));
    assert_eq!(v4.xywy(), V4(101, 102, 104, 102));
    v4.set_xywz(V4(105, 106, 107, 108));
    assert_eq!(v4.xywz(), V4(105, 106, 107, 108));
    assert_eq!(v4.xyww(), V4(105, 106, 107, 107));
    assert_eq!(v4.xzxx(), V4(105, 108, 105, 105));
    assert_eq!(v4.xzxy(), V4(105, 108, 105, 106));
    assert_eq!(v4.xzxz(), V4(105, 108, 105, 108));
    assert_eq!(v4.xzxw(), V4(105, 108, 105, 107));
    assert_eq!(v4.xzyx(), V4(105, 108, 106, 105));
    assert_eq!(v4.xzyy(), V4(105, 108, 106, 106));
    assert_eq!(v4.xzyz(), V4(105, 108, 106, 108));
    v4.set_xzyw(V4(109, 110, 111, 112));
    assert_eq!(v4.xzyw(), V4(109, 110, 111, 112));
    assert_eq!(v4.xzzx(), V4(109, 110, 110, 109));
    assert_eq!(v4.xzzy(), V4(109, 110, 110, 111));
    assert_eq!(v4.xzzz(), V4(109, 110, 110, 110));
    assert_eq!(v4.xzzw(), V4(109, 110, 110, 112));
    assert_eq!(v4.xzwx(), V4(109, 110, 112, 109));
    v4.set_xzwy(V4(113, 114, 115, 116));
    assert_eq!(v4.xzwy(), V4(113, 114, 115, 116));
    assert_eq!(v4.xzwz(), V4(113, 114, 115, 114));
    assert_eq!(v4.xzww(), V4(113, 114, 115, 115));
    assert_eq!(v4.xwxx(), V4(113, 115, 113, 113));
    assert_eq!(v4.xwxy(), V4(113, 115, 113, 116));
    assert_eq!(v4.xwxz(), V4(113, 115, 113, 114));
    assert_eq!(v4.xwxw(), V4(113, 115, 113, 115));
    assert_eq!(v4.xwyx(), V4(113, 115, 116, 113));
    assert_eq!(v4.xwyy(), V4(113, 115, 116, 116));
    v4.set_xwyz(V4(117, 118, 119, 120));
    assert_eq!(v4.xwyz(), V4(117, 118, 119, 120));
    assert_eq!(v4.xwyw(), V4(117, 118, 119, 118));
    assert_eq!(v4.xwzx(), V4(117, 118, 120, 117));
    v4.set_xwzy(V4(121, 122, 123, 124));
    assert_eq!(v4.xwzy(), V4(121, 122, 123, 124));
    assert_eq!(v4.xwzz(), V4(121, 122, 123, 123));
    assert_eq!(v4.xwzw(), V4(121, 122, 123, 122));
    assert_eq!(v4.xwwx(), V4(121, 122, 122, 121));
    assert_eq!(v4.xwwy(), V4(121, 122, 122, 124));
    assert_eq!(v4.xwwz(), V4(121, 122, 122, 123));
    assert_eq!(v4.xwww(), V4(121, 122, 122, 122));
    assert_eq!(v4.yxxx(), V4(124, 121, 121, 121));
    assert_eq!(v4.yxxy(), V4(124, 121, 121, 124));
    assert_eq!(v4.yxxz(), V4(124, 121, 121, 123));
    assert_eq!(v4.yxxw(), V4(124, 121, 121, 122));
    assert_eq!(v4.yxyx(), V4(124, 121, 124, 121));
    assert_eq!(v4.yxyy(), V4(124, 121, 124, 124));
    assert_eq!(v4.yxyz(), V4(124, 121, 124, 123));
    assert_eq!(v4.yxyw(), V4(124, 121, 124, 122));
    assert_eq!(v4.yxzx(), V4(124, 121, 123, 121));
    assert_eq!(v4.yxzy(), V4(124, 121, 123, 124));
    assert_eq!(v4.yxzz(), V4(124, 121, 123, 123));
    v4.set_yxzw(V4(125, 126, 127, 128));
    assert_eq!(v4.yxzw(), V4(125, 126, 127, 128));
    assert_eq!(v4.yxwx(), V4(125, 126, 128, 126));
    assert_eq!(v4.yxwy(), V4(125, 126, 128, 125));
    v4.set_yxwz(V4(129, 130, 131, 132));
    assert_eq!(v4.yxwz(), V4(129, 130, 131, 132));
    assert_eq!(v4.yxww(), V4(129, 130, 131, 131));
    assert_eq!(v4.yyxx(), V4(129, 129, 130, 130));
    assert_eq!(v4.yyxy(), V4(129, 129, 130, 129));
    assert_eq!(v4.yyxz(), V4(129, 129, 130, 132));
    assert_eq!(v4.yyxw(), V4(129, 129, 130, 131));
    assert_eq!(v4.yyyx(), V4(129, 129, 129, 130));
    assert_eq!(v4.yyyy(), V4(129, 129, 129, 129));
    assert_eq!(v4.yyyz(), V4(129, 129, 129, 132));
    assert_eq!(v4.yyyw(), V4(129, 129, 129, 131));
    assert_eq!(v4.yyzx(), V4(129, 129, 132, 130));
    assert_eq!(v4.yyzy(), V4(129, 129, 132, 129));
    assert_eq!(v4.yyzz(), V4(129, 129, 132, 132));
    assert_eq!(v4.yyzw(), V4(129, 129, 132, 131));
    assert_eq!(v4.yywx(), V4(129, 129, 131, 130));
    assert_eq!(v4.yywy(), V4(129, 129, 131, 129));
    assert_eq!(v4.yywz(), V4(129, 129, 131, 132));
    assert_eq!(v4.yyww(), V4(129, 129, 131, 131));
    assert_eq!(v4.yzxx(), V4(129, 132, 130, 130));
    assert_eq!(v4.yzxy(), V4(129, 132, 130, 129));
    assert_eq!(v4.yzxz(), V4(129, 132, 130, 132));
    v4.set_yzxw(V4(133, 134, 135, 136));
    assert_eq!(v4.yzxw(), V4(133, 134, 135, 136));
    assert_eq!(v4.yzyx(), V4(133, 134, 133, 135));
    assert_eq!(v4.yzyy(), V4(133, 134, 133, 133));
    assert_eq!(v4.yzyz(), V4(133, 134, 133, 134));
    assert_eq!(v4.yzyw(), V4(133, 134, 133, 136));
    assert_eq!(v4.yzzx(), V4(133, 134, 134, 135));
    assert_eq!(v4.yzzy(), V4(133, 134, 134, 133));
    assert_eq!(v4.yzzz(), V4(133, 134, 134, 134));
    assert_eq!(v4.yzzw(), V4(133, 134, 134, 136));
    v4.set_yzwx(V4(137, 138, 139, 140));
    assert_eq!(v4.yzwx(), V4(137, 138, 139, 140));
    assert_eq!(v4.yzwy(), V4(137, 138, 139, 137));
    assert_eq!(v4.yzwz(), V4(137, 138, 139, 138));
    assert_eq!(v4.yzww(), V4(137, 138, 139, 139));
    assert_eq!(v4.ywxx(), V4(137, 139, 140, 140));
    assert_eq!(v4.ywxy(), V4(137, 139, 140, 137));
    v4.set_ywxz(V4(141, 142, 143, 144));
    assert_eq!(v4.ywxz(), V4(141, 142, 143, 144));
    assert_eq!(v4.ywxw(), V4(141, 142, 143, 142));
    assert_eq!(v4.ywyx(), V4(141, 142, 141, 143));
    assert_eq!(v4.ywyy(), V4(141, 142, 141, 141));
    assert_eq!(v4.ywyz(), V4(141, 142, 141, 144));
    assert_eq!(v4.ywyw(), V4(141, 142, 141, 142));
    v4.set_ywzx(V4(145, 146, 147, 148));
    assert_eq!(v4.ywzx(), V4(145, 146, 147, 148));
    assert_eq!(v4.ywzy(), V4(145, 146, 147, 145));
    assert_eq!(v4.ywzz(), V4(145, 146, 147, 147));
    assert_eq!(v4.ywzw(), V4(145, 146, 147, 146));
    assert_eq!(v4.ywwx(), V4(145, 146, 146, 148));
    assert_eq!(v4.ywwy(), V4(145, 146, 146, 145));
    assert_eq!(v4.ywwz(), V4(145, 146, 146, 147));
    assert_eq!(v4.ywww(), V4(145, 146, 146, 146));
    assert_eq!(v4.zxxx(), V4(147, 148, 148, 148));
    assert_eq!(v4.zxxy(), V4(147, 148, 148, 145));
    assert_eq!(v4.zxxz(), V4(147, 148, 148, 147));
    assert_eq!(v4.zxxw(), V4(147, 148, 148, 146));
    assert_eq!(v4.zxyx(), V4(147, 148, 145, 148));
    assert_eq!(v4.zxyy(), V4(147, 148, 145, 145));
    assert_eq!(v4.zxyz(), V4(147, 148, 145, 147));
    v4.set_zxyw(V4(149, 150, 151, 152));
    assert_eq!(v4.zxyw(), V4(149, 150, 151, 152));
    assert_eq!(v4.zxzx(), V4(149, 150, 149, 150));
    assert_eq!(v4.zxzy(), V4(149, 150, 149, 151));
    assert_eq!(v4.zxzz(), V4(149, 150, 149, 149));
    assert_eq!(v4.zxzw(), V4(149, 150, 149, 152));
    assert_eq!(v4.zxwx(), V4(149, 150, 152, 150));
    v4.set_zxwy(V4(153, 154, 155, 156));
    assert_eq!(v4.zxwy(), V4(153, 154, 155, 156));
    assert_eq!(v4.zxwz(), V4(153, 154, 155, 153));
    assert_eq!(v4.zxww(), V4(153, 154, 155, 155));
    assert_eq!(v4.zyxx(), V4(153, 156, 154, 154));
    assert_eq!(v4.zyxy(), V4(153, 156, 154, 156));
    assert_eq!(v4.zyxz(), V4(153, 156, 154, 153));
    v4.set_zyxw(V4(157, 158, 159, 160));
    assert_eq!(v4.zyxw(), V4(157, 158, 159, 160));
    assert_eq!(v4.zyyx(), V4(157, 158, 158, 159));
    assert_eq!(v4.zyyy(), V4(157, 158, 158, 158));
    assert_eq!(v4.zyyz(), V4(157, 158, 158, 157));
    assert_eq!(v4.zyyw(), V4(157, 158, 158, 160));
    assert_eq!(v4.zyzx(), V4(157, 158, 157, 159));
    assert_eq!(v4.zyzy(), V4(157, 158, 157, 158));
    assert_eq!(v4.zyzz(), V4(157, 158, 157, 157));
    assert_eq!(v4.zyzw(), V4(157, 158, 157, 160));
    v4.set_zywx(V4(161, 162, 163, 164));
    assert_eq!(v4.zywx(), V4(161, 162, 163, 164));
    assert_eq!(v4.zywy(), V4(161, 162, 163, 162));
    assert_eq!(v4.zywz(), V4(161, 162, 163, 161));
    assert_eq!(v4.zyww(), V4(161, 162, 163, 163));
    assert_eq!(v4.zzxx(), V4(161, 161, 164, 164));
    assert_eq!(v4.zzxy(), V4(161, 161, 164, 162));
    assert_eq!(v4.zzxz(), V4(161, 161, 164, 161));
    assert_eq!(v4.zzxw(), V4(161, 161, 164, 163));
    assert_eq!(v4.zzyx(), V4(161, 161, 162, 164));
    assert_eq!(v4.zzyy(), V4(161, 161, 162, 162));
    assert_eq!(v4.zzyz(), V4(161, 161, 162, 161));
    assert_eq!(v4.zzyw(), V4(161, 161, 162, 163));
    assert_eq!(v4.zzzx(), V4(161, 161, 161, 164));
    assert_eq!(v4.zzzy(), V4(161, 161, 161, 162));
    assert_eq!(v4.zzzz(), V4(161, 161, 161, 161));
    assert_eq!(v4.zzzw(), V4(161, 161, 161, 163));
    assert_eq!(v4.zzwx(), V4(161, 161, 163, 164));
    assert_eq!(v4.zzwy(), V4(161, 161, 163, 162));
    assert_eq!(v4.zzwz(), V4(161, 161, 163, 161));
    assert_eq!(v4.zzww(), V4(161, 161, 163, 163));
    assert_eq!(v4.zwxx(), V4(161, 163, 164, 164));
    v4.set_zwxy(V4(165, 166, 167, 168));
    assert_eq!(v4.zwxy(), V4(165, 166, 167, 168));
    assert_eq!(v4.zwxz(), V4(165, 166, 167, 165));
    assert_eq!(v4.zwxw(), V4(165, 166, 167, 166));
    v4.set_zwyx(V4(169, 170, 171, 172));
    assert_eq!(v4.zwyx(), V4(169, 170, 171, 172));
    assert_eq!(v4.zwyy(), V4(169, 170, 171, 171));
    assert_eq!(v4.zwyz(), V4(169, 170, 171, 169));
    assert_eq!(v4.zwyw(), V4(169, 170, 171, 170));
    assert_eq!(v4.zwzx(), V4(169, 170, 169, 172));
    assert_eq!(v4.zwzy(), V4(169, 170, 169, 171));
    assert_eq!(v4.zwzz(), V4(169, 170, 169, 169));
    assert_eq!(v4.zwzw(), V4(169, 170, 169, 170));
    assert_eq!(v4.zwwx(), V4(169, 170, 170, 172));
    assert_eq!(v4.zwwy(), V4(169, 170, 170, 171));
    assert_eq!(v4.zwwz(), V4(169, 170, 170, 169));
    assert_eq!(v4.zwww(), V4(169, 170, 170, 170));
    assert_eq!(v4.wxxx(), V4(170, 172, 172, 172));
    assert_eq!(v4.wxxy(), V4(170, 172, 172, 171));
    assert_eq!(v4.wxxz(), V4(170, 172, 172, 169));
    assert_eq!(v4.wxxw(), V4(170, 172, 172, 170));
    assert_eq!(v4.wxyx(), V4(170, 172, 171, 172));
    assert_eq!(v4.wxyy(), V4(170, 172, 171, 171));
    v4.set_wxyz(V4(173, 174, 175, 176));
    assert_eq!(v4.wxyz(), V4(173, 174, 175, 176));
    assert_eq!(v4.wxyw(), V4(173, 174, 175, 173));
    assert_eq!(v4.wxzx(), V4(173, 174, 176, 174));
    v4.set_wxzy(V4(177, 178, 179, 180));
    assert_eq!(v4.wxzy(), V4(177, 178, 179, 180));
    assert_eq!(v4.wxzz(), V4(177, 178, 179, 179));
    assert_eq!(v4.wxzw(), V4(177, 178, 179, 177));
    assert_eq!(v4.wxwx(), V4(177, 178, 177, 178));
    assert_eq!(v4.wxwy(), V4(177, 178, 177, 180));
    assert_eq!(v4.wxwz(), V4(177, 178, 177, 179));
    assert_eq!(v4.wxww(), V4(177, 178, 177, 177));
    assert_eq!(v4.wyxx(), V4(177, 180, 178, 178));
    assert_eq!(v4.wyxy(), V4(177, 180, 178, 180));
    v4.set_wyxz(V4(181, 182, 183, 184));
    assert_eq!(v4.wyxz(), V4(181, 182, 183, 184));
    assert_eq!(v4.wyxw(), V4(181, 182, 183, 181));
    assert_eq!(v4.wyyx(), V4(181, 182, 182, 183));
    assert_eq!(v4.wyyy(), V4(181, 182, 182, 182));
    assert_eq!(v4.wyyz(), V4(181, 182, 182, 184));
    assert_eq!(v4.wyyw(), V4(181, 182, 182, 181));
    v4.set_wyzx(V4(185, 186, 187, 188));
    assert_eq!(v4.wyzx(), V4(185, 186, 187, 188));
    assert_eq!(v4.wyzy(), V4(185, 186, 187, 186));
    assert_eq!(v4.wyzz(), V4(185, 186, 187, 187));
    assert_eq!(v4.wyzw(), V4(185, 186, 187, 185));
    assert_eq!(v4.wywx(), V4(185, 186, 185, 188));
    assert_eq!(v4.wywy(), V4(185, 186, 185, 186));
    assert_eq!(v4.wywz(), V4(185, 186, 185, 187));
    assert_eq!(v4.wyww(), V4(185, 186, 185, 185));
    assert_eq!(v4.wzxx(), V4(185, 187, 188, 188));
    v4.set_wzxy(V4(189, 190, 191, 192));
    assert_eq!(v4.wzxy(), V4(189, 190, 191, 192));
    assert_eq!(v4.wzxz(), V4(189, 190, 191, 190));
    assert_eq!(v4.wzxw(), V4(189, 190, 191, 189));
    v4.set_wzyx(V4(193, 194, 195, 196));
    assert_eq!(v4.wzyx(), V4(193, 194, 195, 196));
    assert_eq!(v4.wzyy(), V4(193, 194, 195, 195));
    assert_eq!(v4.wzyz(), V4(193, 194, 195, 194));
    assert_eq!(v4.wzyw(), V4(193, 194, 195, 193));
    assert_eq!(v4.wzzx(), V4(193, 194, 194, 196));
    assert_eq!(v4.wzzy(), V4(193, 194, 194, 195));
    assert_eq!(v4.wzzz(), V4(193, 194, 194, 194));
    assert_eq!(v4.wzzw(), V4(193, 194, 194, 193));
    assert_eq!(v4.wzwx(), V4(193, 194, 193, 196));
    assert_eq!(v4.wzwy(), V4(193, 194, 193, 195));
    assert_eq!(v4.wzwz(), V4(193, 194, 193, 194));
    assert_eq!(v4.wzww(), V4(193, 194, 193, 193));
    assert_eq!(v4.wwxx(), V4(193, 193, 196, 196));
    assert_eq!(v4.wwxy(), V4(193, 193, 196, 195));
    assert_eq!(v4.wwxz(), V4(193, 193, 196, 194));
    assert_eq!(v4.wwxw(), V4(193, 193, 196, 193));
    assert_eq!(v4.wwyx(), V4(193, 193, 195, 196));
    assert_eq!(v4.wwyy(), V4(193, 193, 195, 195));
    assert_eq!(v4.wwyz(), V4(193, 193, 195, 194));
    assert_eq!(v4.wwyw(), V4(193, 193, 195, 193));
    assert_eq!(v4.wwzx(), V4(193, 193, 194, 196));
    assert_eq!(v4.wwzy(), V4(193, 193, 194, 195));
    assert_eq!(v4.wwzz(), V4(193, 193, 194, 194));
    assert_eq!(v4.wwzw(), V4(193, 193, 194, 193));
    assert_eq!(v4.wwwx(), V4(193, 193, 193, 196));
    assert_eq!(v4.wwwy(), V4(193, 193, 193, 195));
    assert_eq!(v4.wwwz(), V4(193, 193, 193, 194));
    assert_eq!(v4.wwww(), V4(193, 193, 193, 193));
    v4.set_r(197);
    assert_eq!(v4.r(), 197);
    v4.set_g(198);
    assert_eq!(v4.g(), 198);
    v4.set_b(199);
    assert_eq!(v4.b(), 199);
    v4.set_a(200);
    assert_eq!(v4.a(), 200);
    assert_eq!(v4.rr(), V2(197, 197));
    v4.set_rg(V2(201, 202));
    assert_eq!(v4.rg(), V2(201, 202));
    v4.set_rb(V2(203, 204));
    assert_eq!(v4.rb(), V2(203, 204));
    v4.set_ra(V2(205, 206));
    assert_eq!(v4.ra(), V2(205, 206));
    v4.set_gr(V2(207, 208));
    assert_eq!(v4.gr(), V2(207, 208));
    assert_eq!(v4.gg(), V2(207, 207));
    v4.set_gb(V2(209, 210));
    assert_eq!(v4.gb(), V2(209, 210));
    v4.set_ga(V2(211, 212));
    assert_eq!(v4.ga(), V2(211, 212));
    v4.set_br(V2(213, 214));
    assert_eq!(v4.br(), V2(213, 214));
    v4.set_bg(V2(215, 216));
    assert_eq!(v4.bg(), V2(215, 216));
    assert_eq!(v4.bb(), V2(215, 215));
    v4.set_ba(V2(217, 218));
    assert_eq!(v4.ba(), V2(217, 218));
    v4.set_ar(V2(219, 220));
    assert_eq!(v4.ar(), V2(219, 220));
    v4.set_ag(V2(221, 222));
    assert_eq!(v4.ag(), V2(221, 222));
    v4.set_ab(V2(223, 224));
    assert_eq!(v4.ab(), V2(223, 224));
    assert_eq!(v4.aa(), V2(223, 223));
    assert_eq!(v4.rrr(), V3(220, 220, 220));
    assert_eq!(v4.rrg(), V3(220, 220, 222));
    assert_eq!(v4.rrb(), V3(220, 220, 224));
    assert_eq!(v4.rra(), V3(220, 220, 223));
    assert_eq!(v4.rgr(), V3(220, 222, 220));
    assert_eq!(v4.rgg(), V3(220, 222, 222));
    v4.set_rgb(V3(225, 226, 227));
    assert_eq!(v4.rgb(), V3(225, 226, 227));
    v4.set_rga(V3(228, 229, 230));
    assert_eq!(v4.rga(), V3(228, 229, 230));
    assert_eq!(v4.rbr(), V3(228, 227, 228));
    v4.set_rbg(V3(231, 232, 233));
    assert_eq!(v4.rbg(), V3(231, 232, 233));
    assert_eq!(v4.rbb(), V3(231, 232, 232));
    v4.set_rba(V3(234, 235, 236));
    assert_eq!(v4.rba(), V3(234, 235, 236));
    assert_eq!(v4.rar(), V3(234, 236, 234));
    v4.set_rag(V3(237, 238, 239));
    assert_eq!(v4.rag(), V3(237, 238, 239));
    v4.set_rab(V3(240, 241, 242));
    assert_eq!(v4.rab(), V3(240, 241, 242));
    assert_eq!(v4.raa(), V3(240, 241, 241));
    assert_eq!(v4.grr(), V3(239, 240, 240));
    assert_eq!(v4.grg(), V3(239, 240, 239));
    v4.set_grb(V3(243, 244, 245));
    assert_eq!(v4.grb(), V3(243, 244, 245));
    v4.set_gra(V3(246, 247, 248));
    assert_eq!(v4.gra(), V3(246, 247, 248));
    assert_eq!(v4.ggr(), V3(246, 246, 247));
    assert_eq!(v4.ggg(), V3(246, 246, 246));
    assert_eq!(v4.ggb(), V3(246, 246, 245));
    assert_eq!(v4.gga(), V3(246, 246, 248));
    v4.set_gbr(V3(249, 250, 251));
    assert_eq!(v4.gbr(), V3(249, 250, 251));
    assert_eq!(v4.gbg(), V3(249, 250, 249));
    assert_eq!(v4.gbb(), V3(249, 250, 250));
    v4.set_gba(V3(252, 253, 254));
    assert_eq!(v4.gba(), V3(252, 253, 254));
    v4.set_gar(V3(255, 256, 257));
    assert_eq!(v4.gar(), V3(255, 256, 257));
    assert_eq!(v4.gag(), V3(255, 256, 255));
    v4.set_gab(V3(258, 259, 260));
    assert_eq!(v4.gab(), V3(258, 259, 260));
    assert_eq!(v4.gaa(), V3(258, 259, 259));
    assert_eq!(v4.brr(), V3(260, 257, 257));
    v4.set_brg(V3(261, 262, 263));
    assert_eq!(v4.brg(), V3(261, 262, 263));
    assert_eq!(v4.brb(), V3(261, 262, 261));
    v4.set_bra(V3(264, 265, 266));
    assert_eq!(v4.bra(), V3(264, 265, 266));
    v4.set_bgr(V3(267, 268, 269));
    assert_eq!(v4.bgr(), V3(267, 268, 269));
    assert_eq!(v4.bgg(), V3(267, 268, 268));
    assert_eq!(v4.bgb(), V3(267, 268, 267));
    v4.set_bga(V3(270, 271, 272));
    assert_eq!(v4.bga(), V3(270, 271, 272));
    assert_eq!(v4.bbr(), V3(270, 270, 269));
    assert_eq!(v4.bbg(), V3(270, 270, 271));
    assert_eq!(v4.bbb(), V3(270, 270, 270));
    assert_eq!(v4.bba(), V3(270, 270, 272));
    v4.set_bar(V3(273, 274, 275));
    assert_eq!(v4.bar(), V3(273, 274, 275));
    v4.set_bag(V3(276, 277, 278));
    assert_eq!(v4.bag(), V3(276, 277, 278));
    assert_eq!(v4.bab(), V3(276, 277, 276));
    assert_eq!(v4.baa(), V3(276, 277, 277));
    assert_eq!(v4.arr(), V3(277, 275, 275));
    v4.set_arg(V3(279, 280, 281));
    assert_eq!(v4.arg(), V3(279, 280, 281));
    v4.set_arb(V3(282, 283, 284));
    assert_eq!(v4.arb(), V3(282, 283, 284));
    assert_eq!(v4.ara(), V3(282, 283, 282));
    v4.set_agr(V3(285, 286, 287));
    assert_eq!(v4.agr(), V3(285, 286, 287));
    assert_eq!(v4.agg(), V3(285, 286, 286));
    v4.set_agb(V3(288, 289, 290));
    assert_eq!(v4.agb(), V3(288, 289, 290));
    assert_eq!(v4.aga(), V3(288, 289, 288));
    v4.set_abr(V3(291, 292, 293));
    assert_eq!(v4.abr(), V3(291, 292, 293));
    v4.set_abg(V3(294, 295, 296));
    assert_eq!(v4.abg(), V3(294, 295, 296));
    assert_eq!(v4.abb(), V3(294, 295, 295));
    assert_eq!(v4.aba(), V3(294, 295, 294));
    assert_eq!(v4.aar(), V3(294, 294, 293));
    assert_eq!(v4.aag(), V3(294, 294, 296));
    assert_eq!(v4.aab(), V3(294, 294, 295));
    assert_eq!(v4.aaa(), V3(294, 294, 294));
    assert_eq!(v4.rrrr(), V4(293, 293, 293, 293));
    assert_eq!(v4.rrrg(), V4(293, 293, 293, 296));
    assert_eq!(v4.rrrb(), V4(293, 293, 293, 295));
    assert_eq!(v4.rrra(), V4(293, 293, 293, 294));
    assert_eq!(v4.rrgr(), V4(293, 293, 296, 293));
    assert_eq!(v4.rrgg(), V4(293, 293, 296, 296));
    assert_eq!(v4.rrgb(), V4(293, 293, 296, 295));
    assert_eq!(v4.rrga(), V4(293, 293, 296, 294));
    assert_eq!(v4.rrbr(), V4(293, 293, 295, 293));
    assert_eq!(v4.rrbg(), V4(293, 293, 295, 296));
    assert_eq!(v4.rrbb(), V4(293, 293, 295, 295));
    assert_eq!(v4.rrba(), V4(293, 293, 295, 294));
    assert_eq!(v4.rrar(), V4(293, 293, 294, 293));
    assert_eq!(v4.rrag(), V4(293, 293, 294, 296));
    assert_eq!(v4.rrab(), V4(293, 293, 294, 295));
    assert_eq!(v4.rraa(), V4(293, 293, 294, 294));
    assert_eq!(v4.rgrr(), V4(293, 296, 293, 293));
    assert_eq!(v4.rgrg(), V4(293, 296, 293, 296));
    assert_eq!(v4.rgrb(), V4(293, 296, 293, 295));
    assert_eq!(v4.rgra(), V4(293, 296, 293, 294));
    assert_eq!(v4.rggr(), V4(293, 296, 296, 293));
    assert_eq!(v4.rggg(), V4(293, 296, 296, 296));
    assert_eq!(v4.rggb(), V4(293, 296, 296, 295));
    assert_eq!(v4.rgga(), V4(293, 296, 296, 294));
    assert_eq!(v4.rgbr(), V4(293, 296, 295, 293));
    assert_eq!(v4.rgbg(), V4(293, 296, 295, 296));
    assert_eq!(v4.rgbb(), V4(293, 296, 295, 295));
    v4.set_rgba(V4(297, 298, 299, 300));
    assert_eq!(v4.rgba(), V4(297, 298, 299, 300));
    assert_eq!(v4.rgar(), V4(297, 298, 300, 297));
    assert_eq!(v4.rgag(), V4(297, 298, 300, 298));
    v4.set_rgab(V4(301, 302, 303, 304));
    assert_eq!(v4.rgab(), V4(301, 302, 303, 304));
    assert_eq!(v4.rgaa(), V4(301, 302, 303, 303));
    assert_eq!(v4.rbrr(), V4(301, 304, 301, 301));
    assert_eq!(v4.rbrg(), V4(301, 304, 301, 302));
    assert_eq!(v4.rbrb(), V4(301, 304, 301, 304));
    assert_eq!(v4.rbra(), V4(301, 304, 301, 303));
    assert_eq!(v4.rbgr(), V4(301, 304, 302, 301));
    assert_eq!(v4.rbgg(), V4(301, 304, 302, 302));
    assert_eq!(v4.rbgb(), V4(301, 304, 302, 304));
    v4.set_rbga(V4(305, 306, 307, 308));
    assert_eq!(v4.rbga(), V4(305, 306, 307, 308));
    assert_eq!(v4.rbbr(), V4(305, 306, 306, 305));
    assert_eq!(v4.rbbg(), V4(305, 306, 306, 307));
    assert_eq!(v4.rbbb(), V4(305, 306, 306, 306));
    assert_eq!(v4.rbba(), V4(305, 306, 306, 308));
    assert_eq!(v4.rbar(), V4(305, 306, 308, 305));
    v4.set_rbag(V4(309, 310, 311, 312));
    assert_eq!(v4.rbag(), V4(309, 310, 311, 312));
    assert_eq!(v4.rbab(), V4(309, 310, 311, 310));
    assert_eq!(v4.rbaa(), V4(309, 310, 311, 311));
    assert_eq!(v4.rarr(), V4(309, 311, 309, 309));
    assert_eq!(v4.rarg(), V4(309, 311, 309, 312));
    assert_eq!(v4.rarb(), V4(309, 311, 309, 310));
    assert_eq!(v4.rara(), V4(309, 311, 309, 311));
    assert_eq!(v4.ragr(), V4(309, 311, 312, 309));
    assert_eq!(v4.ragg(), V4(309, 311, 312, 312));
    v4.set_ragb(V4(313, 314, 315, 316));
    assert_eq!(v4.ragb(), V4(313, 314, 315, 316));
    assert_eq!(v4.raga(), V4(313, 314, 315, 314));
    assert_eq!(v4.rabr(), V4(313, 314, 316, 313));
    v4.set_rabg(V4(317, 318, 319, 320));
    assert_eq!(v4.rabg(), V4(317, 318, 319, 320));
    assert_eq!(v4.rabb(), V4(317, 318, 319, 319));
    assert_eq!(v4.raba(), V4(317, 318, 319, 318));
    assert_eq!(v4.raar(), V4(317, 318, 318, 317));
    assert_eq!(v4.raag(), V4(317, 318, 318, 320));
    assert_eq!(v4.raab(), V4(317, 318, 318, 319));
    assert_eq!(v4.raaa(), V4(317, 318, 318, 318));
    assert_eq!(v4.grrr(), V4(320, 317, 317, 317));
    assert_eq!(v4.grrg(), V4(320, 317, 317, 320));
    assert_eq!(v4.grrb(), V4(320, 317, 317, 319));
    assert_eq!(v4.grra(), V4(320, 317, 317, 318));
    assert_eq!(v4.grgr(), V4(320, 317, 320, 317));
    assert_eq!(v4.grgg(), V4(320, 317, 320, 320));
    assert_eq!(v4.grgb(), V4(320, 317, 320, 319));
    assert_eq!(v4.grga(), V4(320, 317, 320, 318));
    assert_eq!(v4.grbr(), V4(320, 317, 319, 317));
    assert_eq!(v4.grbg(), V4(320, 317, 319, 320));
    assert_eq!(v4.grbb(), V4(320, 317, 319, 319));
    v4.set_grba(V4(321, 322, 323, 324));
    assert_eq!(v4.grba(), V4(321, 322, 323, 324));
    assert_eq!(v4.grar(), V4(321, 322, 324, 322));
    assert_eq!(v4.grag(), V4(321, 322, 324, 321));
    v4.set_grab(V4(325, 326, 327, 328));
    assert_eq!(v4.grab(), V4(325, 326, 327, 328));
    assert_eq!(v4.graa(), V4(325, 326, 327, 327));
    assert_eq!(v4.ggrr(), V4(325, 325, 326, 326));
    assert_eq!(v4.ggrg(), V4(325, 325, 326, 325));
    assert_eq!(v4.ggrb(), V4(325, 325, 326, 328));
    assert_eq!(v4.ggra(), V4(325, 325, 326, 327));
    assert_eq!(v4.gggr(), V4(325, 325, 325, 326));
    assert_eq!(v4.gggg(), V4(325, 325, 325, 325));
    assert_eq!(v4.gggb(), V4(325, 325, 325, 328));
    assert_eq!(v4.ggga(), V4(325, 325, 325, 327));
    assert_eq!(v4.ggbr(), V4(325, 325, 328, 326));
    assert_eq!(v4.ggbg(), V4(325, 325, 328, 325));
    assert_eq!(v4.ggbb(), V4(325, 325, 328, 328));
    assert_eq!(v4.ggba(), V4(325, 325, 328, 327));
    assert_eq!(v4.ggar(), V4(325, 325, 327, 326));
    assert_eq!(v4.ggag(), V4(325, 325, 327, 325));
    assert_eq!(v4.ggab(), V4(325, 325, 327, 328));
    assert_eq!(v4.ggaa(), V4(325, 325, 327, 327));
    assert_eq!(v4.gbrr(), V4(325, 328, 326, 326));
    assert_eq!(v4.gbrg(), V4(325, 328, 326, 325));
    assert_eq!(v4.gbrb(), V4(325, 328, 326, 328));
    v4.set_gbra(V4(329, 330, 331, 332));
    assert_eq!(v4.gbra(), V4(329, 330, 331, 332));
    assert_eq!(v4.gbgr(), V4(329, 330, 329, 331));
    assert_eq!(v4.gbgg(), V4(329, 330, 329, 329));
    assert_eq!(v4.gbgb(), V4(329, 330, 329, 330));
    assert_eq!(v4.gbga(), V4(329, 330, 329, 332));
    assert_eq!(v4.gbbr(), V4(329, 330, 330, 331));
    assert_eq!(v4.gbbg(), V4(329, 330, 330, 329));
    assert_eq!(v4.gbbb(), V4(329, 330, 330, 330));
    assert_eq!(v4.gbba(), V4(329, 330, 330, 332));
    v4.set_gbar(V4(333, 334, 335, 336));
    assert_eq!(v4.gbar(), V4(333, 334, 335, 336));
    assert_eq!(v4.gbag(), V4(333, 334, 335, 333));
    assert_eq!(v4.gbab(), V4(333, 334, 335, 334));
    assert_eq!(v4.gbaa(), V4(333, 334, 335, 335));
    assert_eq!(v4.garr(), V4(333, 335, 336, 336));
    assert_eq!(v4.garg(), V4(333, 335, 336, 333));
    v4.set_garb(V4(337, 338, 339, 340));
    assert_eq!(v4.garb(), V4(337, 338, 339, 340));
    assert_eq!(v4.gara(), V4(337, 338, 339, 338));
    assert_eq!(v4.gagr(), V4(337, 338, 337, 339));
    assert_eq!(v4.gagg(), V4(337, 338, 337, 337));
    assert_eq!(v4.gagb(), V4(337, 338, 337, 340));
    assert_eq!(v4.gaga(), V4(337, 338, 337, 338));
    v4.set_gabr(V4(341, 342, 343, 344));
    assert_eq!(v4.gabr(), V4(341, 342, 343, 344));
    assert_eq!(v4.gabg(), V4(341, 342, 343, 341));
    assert_eq!(v4.gabb(), V4(341, 342, 343, 343));
    assert_eq!(v4.gaba(), V4(341, 342, 343, 342));
    assert_eq!(v4.gaar(), V4(341, 342, 342, 344));
    assert_eq!(v4.gaag(), V4(341, 342, 342, 341));
    assert_eq!(v4.gaab(), V4(341, 342, 342, 343));
    assert_eq!(v4.gaaa(), V4(341, 342, 342, 342));
    assert_eq!(v4.brrr(), V4(343, 344, 344, 344));
    assert_eq!(v4.brrg(), V4(343, 344, 344, 341));
    assert_eq!(v4.brrb(), V4(343, 344, 344, 343));
    assert_eq!(v4.brra(), V4(343, 344, 344, 342));
    assert_eq!(v4.brgr(), V4(343, 344, 341, 344));
    assert_eq!(v4.brgg(), V4(343, 344, 341, 341));
    assert_eq!(v4.brgb(), V4(343, 344, 341, 343));
    v4.set_brga(V4(345, 346, 347, 348));
    assert_eq!(v4.brga(), V4(345, 346, 347, 348));
    assert_eq!(v4.brbr(), V4(345, 346, 345, 346));
    assert_eq!(v4.brbg(), V4(345, 346, 345, 347));
    assert_eq!(v4.brbb(), V4(345, 346, 345, 345));
    assert_eq!(v4.brba(), V4(345, 346, 345, 348));
    assert_eq!(v4.brar(), V4(345, 346, 348, 346));
    v4.set_brag(V4(349, 350, 351, 352));
    assert_eq!(v4.brag(), V4(349, 350, 351, 352));
    assert_eq!(v4.brab(), V4(349, 350, 351, 349));
    assert_eq!(v4.braa(), V4(349, 350, 351, 351));
    assert_eq!(v4.bgrr(), V4(349, 352, 350, 350));
    assert_eq!(v4.bgrg(), V4(349, 352, 350, 352));
    assert_eq!(v4.bgrb(), V4(349, 352, 350, 349));
    v4.set_bgra(V4(353, 354, 355, 356));
    assert_eq!(v4.bgra(), V4(353, 354, 355, 356));
    assert_eq!(v4.bggr(), V4(353, 354, 354, 355));
    assert_eq!(v4.bggg(), V4(353, 354, 354, 354));
    assert_eq!(v4.bggb(), V4(353, 354, 354, 353));
    assert_eq!(v4.bgga(), V4(353, 354, 354, 356));
    assert_eq!(v4.bgbr(), V4(353, 354, 353, 355));
    assert_eq!(v4.bgbg(), V4(353, 354, 353, 354));
    assert_eq!(v4.bgbb(), V4(353, 354, 353, 353));
    assert_eq!(v4.bgba(), V4(353, 354, 353, 356));
    v4.set_bgar(V4(357, 358, 359, 360));
    assert_eq!(v4.bgar(), V4(357, 358, 359, 360));
    assert_eq!(v4.bgag(), V4(357, 358, 359, 358));
    assert_eq!(v4.bgab(), V4(357, 358, 359, 357));
    assert_eq!(v4.bgaa(), V4(357, 358, 359, 359));
    assert_eq!(v4.bbrr(), V4(357, 357, 360, 360));
    assert_eq!(v4.bbrg(), V4(357, 357, 360, 358));
    assert_eq!(v4.bbrb(), V4(357, 357, 360, 357));
    assert_eq!(v4.bbra(), V4(357, 357, 360, 359));
    assert_eq!(v4.bbgr(), V4(357, 357, 358, 360));
    assert_eq!(v4.bbgg(), V4(357, 357, 358, 358));
    assert_eq!(v4.bbgb(), V4(357, 357, 358, 357));
    assert_eq!(v4.bbga(), V4(357, 357, 358, 359));
    assert_eq!(v4.bbbr(), V4(357, 357, 357, 360));
    assert_eq!(v4.bbbg(), V4(357, 357, 357, 358));
    assert_eq!(v4.bbbb(), V4(357, 357, 357, 357));
    assert_eq!(v4.bbba(), V4(357, 357, 357, 359));
    assert_eq!(v4.bbar(), V4(357, 357, 359, 360));
    assert_eq!(v4.bbag(), V4(357, 357, 359, 358));
    assert_eq!(v4.bbab(), V4(357, 357, 359, 357));
    assert_eq!(v4.bbaa(), V4(357, 357, 359, 359));
    assert_eq!(v4.barr(), V4(357, 359, 360, 360));
    v4.set_barg(V4(361, 362, 363, 364));
    assert_eq!(v4.barg(), V4(361, 362, 363, 364));
    assert_eq!(v4.barb(), V4(361, 362, 363, 361));
    assert_eq!(v4.bara(), V4(361, 362, 363, 362));
    v4.set_bagr(V4(365, 366, 367, 368));
    assert_eq!(v4.bagr(), V4(365, 366, 367, 368));
    assert_eq!(v4.bagg(), V4(365, 366, 367, 367));
    assert_eq!(v4.bagb(), V4(365, 366, 367, 365));
    assert_eq!(v4.baga(), V4(365, 366, 367, 366));
    assert_eq!(v4.babr(), V4(365, 366, 365, 368));
    assert_eq!(v4.babg(), V4(365, 366, 365, 367));
    assert_eq!(v4.babb(), V4(365, 366, 365, 365));
    assert_eq!(v4.baba(), V4(365, 366, 365, 366));
    assert_eq!(v4.baar(), V4(365, 366, 366, 368));
    assert_eq!(v4.baag(), V4(365, 366, 366, 367));
    assert_eq!(v4.baab(), V4(365, 366, 366, 365));
    assert_eq!(v4.baaa(), V4(365, 366, 366, 366));
    assert_eq!(v4.arrr(), V4(366, 368, 368, 368));
    assert_eq!(v4.arrg(), V4(366, 368, 368, 367));
    assert_eq!(v4.arrb(), V4(366, 368, 368, 365));
    assert_eq!(v4.arra(), V4(366, 368, 368, 366));
    assert_eq!(v4.argr(), V4(366, 368, 367, 368));
    assert_eq!(v4.argg(), V4(366, 368, 367, 367));
    v4.set_argb(V4(369, 370, 371, 372));
    assert_eq!(v4.argb(), V4(369, 370, 371, 372));
    assert_eq!(v4.arga(), V4(369, 370, 371, 369));
    assert_eq!(v4.arbr(), V4(369, 370, 372, 370));
    v4.set_arbg(V4(373, 374, 375, 376));
    assert_eq!(v4.arbg(), V4(373, 374, 375, 376));
    assert_eq!(v4.arbb(), V4(373, 374, 375, 375));
    assert_eq!(v4.arba(), V4(373, 374, 375, 373));
    assert_eq!(v4.arar(), V4(373, 374, 373, 374));
    assert_eq!(v4.arag(), V4(373, 374, 373, 376));
    assert_eq!(v4.arab(), V4(373, 374, 373, 375));
    assert_eq!(v4.araa(), V4(373, 374, 373, 373));
    assert_eq!(v4.agrr(), V4(373, 376, 374, 374));
    assert_eq!(v4.agrg(), V4(373, 376, 374, 376));
    v4.set_agrb(V4(377, 378, 379, 380));
    assert_eq!(v4.agrb(), V4(377, 378, 379, 380));
    assert_eq!(v4.agra(), V4(377, 378, 379, 377));
    assert_eq!(v4.aggr(), V4(377, 378, 378, 379));
    assert_eq!(v4.aggg(), V4(377, 378, 378, 378));
    assert_eq!(v4.aggb(), V4(377, 378, 378, 380));
    assert_eq!(v4.agga(), V4(377, 378, 378, 377));
    v4.set_agbr(V4(381, 382, 383, 384));
    assert_eq!(v4.agbr(), V4(381, 382, 383, 384));
    assert_eq!(v4.agbg(), V4(381, 382, 383, 382));
    assert_eq!(v4.agbb(), V4(381, 382, 383, 383));
    assert_eq!(v4.agba(), V4(381, 382, 383, 381));
    assert_eq!(v4.agar(), V4(381, 382, 381, 384));
    assert_eq!(v4.agag(), V4(381, 382, 381, 382));
    assert_eq!(v4.agab(), V4(381, 382, 381, 383));
    assert_eq!(v4.agaa(), V4(381, 382, 381, 381));
    assert_eq!(v4.abrr(), V4(381, 383, 384, 384));
    v4.set_abrg(V4(385, 386, 387, 388));
    assert_eq!(v4.abrg(), V4(385, 386, 387, 388));
    assert_eq!(v4.abrb(), V4(385, 386, 387, 386));
    assert_eq!(v4.abra(), V4(385, 386, 387, 385));
    v4.set_abgr(V4(389, 390, 391, 392));
    assert_eq!(v4.abgr(), V4(389, 390, 391, 392));
    assert_eq!(v4.abgg(), V4(389, 390, 391, 391));
    assert_eq!(v4.abgb(), V4(389, 390, 391, 390));
    assert_eq!(v4.abga(), V4(389, 390, 391, 389));
    assert_eq!(v4.abbr(), V4(389, 390, 390, 392));
    assert_eq!(v4.abbg(), V4(389, 390, 390, 391));
    assert_eq!(v4.abbb(), V4(389, 390, 390, 390));
    assert_eq!(v4.abba(), V4(389, 390, 390, 389));
    assert_eq!(v4.abar(), V4(389, 390, 389, 392));
    assert_eq!(v4.abag(), V4(389, 390, 389, 391));
    assert_eq!(v4.abab(), V4(389, 390, 389, 390));
    assert_eq!(v4.abaa(), V4(389, 390, 389, 389));
    assert_eq!(v4.aarr(), V4(389, 389, 392, 392));
    assert_eq!(v4.aarg(), V4(389, 389, 392, 391));
    assert_eq!(v4.aarb(), V4(389, 389, 392, 390));
    assert_eq!(v4.aara(), V4(389, 389, 392, 389));
    assert_eq!(v4.aagr(), V4(389, 389, 391, 392));
    assert_eq!(v4.aagg(), V4(389, 389, 391, 391));
    assert_eq!(v4.aagb(), V4(389, 389, 391, 390));
    assert_eq!(v4.aaga(), V4(389, 389, 391, 389));
    assert_eq!(v4.aabr(), V4(389, 389, 390, 392));
    assert_eq!(v4.aabg(), V4(389, 389, 390, 391));
    assert_eq!(v4.aabb(), V4(389, 389, 390, 390));
    assert_eq!(v4.aaba(), V4(389, 389, 390, 389));
    assert_eq!(v4.aaar(), V4(389, 389, 389, 392));
    assert_eq!(v4.aaag(), V4(389, 389, 389, 391));
    assert_eq!(v4.aaab(), V4(389, 389, 389, 390));
    assert_eq!(v4.aaaa(), V4(389, 389, 389, 389));
    v4.set_s(393);
    assert_eq!(v4.s(), 393);
    v4.set_t(394);
    assert_eq!(v4.t(), 394);
    v4.set_p(395);
    assert_eq!(v4.p(), 395);
    v4.set_q(396);
    assert_eq!(v4.q(), 396);
    assert_eq!(v4.ss(), V2(393, 393));
    v4.set_st(V2(397, 398));
    assert_eq!(v4.st(), V2(397, 398));
    v4.set_sp(V2(399, 400));
    assert_eq!(v4.sp(), V2(399, 400));
    v4.set_sq(V2(401, 402));
    assert_eq!(v4.sq(), V2(401, 402));
    v4.set_ts(V2(403, 404));
    assert_eq!(v4.ts(), V2(403, 404));
    assert_eq!(v4.tt(), V2(403, 403));
    v4.set_tp(V2(405, 406));
    assert_eq!(v4.tp(), V2(405, 406));
    v4.set_tq(V2(407, 408));
    assert_eq!(v4.tq(), V2(407, 408));
    v4.set_ps(V2(409, 410));
    assert_eq!(v4.ps(), V2(409, 410));
    v4.set_pt(V2(411, 412));
    assert_eq!(v4.pt(), V2(411, 412));
    assert_eq!(v4.pp(), V2(411, 411));
    v4.set_pq(V2(413, 414));
    assert_eq!(v4.pq(), V2(413, 414));
    v4.set_qs(V2(415, 416));
    assert_eq!(v4.qs(), V2(415, 416));
    v4.set_qt(V2(417, 418));
    assert_eq!(v4.qt(), V2(417, 418));
    v4.set_qp(V2(419, 420));
    assert_eq!(v4.qp(), V2(419, 420));
    assert_eq!(v4.qq(), V2(419, 419));
    assert_eq!(v4.sss(), V3(416, 416, 416));
    assert_eq!(v4.sst(), V3(416, 416, 418));
    assert_eq!(v4.ssp(), V3(416, 416, 420));
    assert_eq!(v4.ssq(), V3(416, 416, 419));
    assert_eq!(v4.sts(), V3(416, 418, 416));
    assert_eq!(v4.stt(), V3(416, 418, 418));
    v4.set_stp(V3(421, 422, 423));
    assert_eq!(v4.stp(), V3(421, 422, 423));
    v4.set_stq(V3(424, 425, 426));
    assert_eq!(v4.stq(), V3(424, 425, 426));
    assert_eq!(v4.sps(), V3(424, 423, 424));
    v4.set_spt(V3(427, 428, 429));
    assert_eq!(v4.spt(), V3(427, 428, 429));
    assert_eq!(v4.spp(), V3(427, 428, 428));
    v4.set_spq(V3(430, 431, 432));
    assert_eq!(v4.spq(), V3(430, 431, 432));
    assert_eq!(v4.sqs(), V3(430, 432, 430));
    v4.set_sqt(V3(433, 434, 435));
    assert_eq!(v4.sqt(), V3(433, 434, 435));
    v4.set_sqp(V3(436, 437, 438));
    assert_eq!(v4.sqp(), V3(436, 437, 438));
    assert_eq!(v4.sqq(), V3(436, 437, 437));
    assert_eq!(v4.tss(), V3(435, 436, 436));
    assert_eq!(v4.tst(), V3(435, 436, 435));
    v4.set_tsp(V3(439, 440, 441));
    assert_eq!(v4.tsp(), V3(439, 440, 441));
    v4.set_tsq(V3(442, 443, 444));
    assert_eq!(v4.tsq(), V3(442, 443, 444));
    assert_eq!(v4.tts(), V3(442, 442, 443));
    assert_eq!(v4.ttt(), V3(442, 442, 442));
    assert_eq!(v4.ttp(), V3(442, 442, 441));
    assert_eq!(v4.ttq(), V3(442, 442, 444));
    v4.set_tps(V3(445, 446, 447));
    assert_eq!(v4.tps(), V3(445, 446, 447));
    assert_eq!(v4.tpt(), V3(445, 446, 445));
    assert_eq!(v4.tpp(), V3(445, 446, 446));
    v4.set_tpq(V3(448, 449, 450));
    assert_eq!(v4.tpq(), V3(448, 449, 450));
    v4.set_tqs(V3(451, 452, 453));
    assert_eq!(v4.tqs(), V3(451, 452, 453));
    assert_eq!(v4.tqt(), V3(451, 452, 451));
    v4.set_tqp(V3(454, 455, 456));
    assert_eq!(v4.tqp(), V3(454, 455, 456));
    assert_eq!(v4.tqq(), V3(454, 455, 455));
    assert_eq!(v4.pss(), V3(456, 453, 453));
    v4.set_pst(V3(457, 458, 459));
    assert_eq!(v4.pst(), V3(457, 458, 459));
    assert_eq!(v4.psp(), V3(457, 458, 457));
    v4.set_psq(V3(460, 461, 462));
    assert_eq!(v4.psq(), V3(460, 461, 462));
    v4.set_pts(V3(463, 464, 465));
    assert_eq!(v4.pts(), V3(463, 464, 465));
    assert_eq!(v4.ptt(), V3(463, 464, 464));
    assert_eq!(v4.ptp(), V3(463, 464, 463));
    v4.set_ptq(V3(466, 467, 468));
    assert_eq!(v4.ptq(), V3(466, 467, 468));
    assert_eq!(v4.pps(), V3(466, 466, 465));
    assert_eq!(v4.ppt(), V3(466, 466, 467));
    assert_eq!(v4.ppp(), V3(466, 466, 466));
    assert_eq!(v4.ppq(), V3(466, 466, 468));
    v4.set_pqs(V3(469, 470, 471));
    assert_eq!(v4.pqs(), V3(469, 470, 471));
    v4.set_pqt(V3(472, 473, 474));
    assert_eq!(v4.pqt(), V3(472, 473, 474));
    assert_eq!(v4.pqp(), V3(472, 473, 472));
    assert_eq!(v4.pqq(), V3(472, 473, 473));
    assert_eq!(v4.qss(), V3(473, 471, 471));
    v4.set_qst(V3(475, 476, 477));
    assert_eq!(v4.qst(), V3(475, 476, 477));
    v4.set_qsp(V3(478, 479, 480));
    assert_eq!(v4.qsp(), V3(478, 479, 480));
    assert_eq!(v4.qsq(), V3(478, 479, 478));
    v4.set_qts(V3(481, 482, 483));
    assert_eq!(v4.qts(), V3(481, 482, 483));
    assert_eq!(v4.qtt(), V3(481, 482, 482));
    v4.set_qtp(V3(484, 485, 486));
    assert_eq!(v4.qtp(), V3(484, 485, 486));
    assert_eq!(v4.qtq(), V3(484, 485, 484));
    v4.set_qps(V3(487, 488, 489));
    assert_eq!(v4.qps(), V3(487, 488, 489));
    v4.set_qpt(V3(490, 491, 492));
    assert_eq!(v4.qpt(), V3(490, 491, 492));
    assert_eq!(v4.qpp(), V3(490, 491, 491));
    assert_eq!(v4.qpq(), V3(490, 491, 490));
    assert_eq!(v4.qqs(), V3(490, 490, 489));
    assert_eq!(v4.qqt(), V3(490, 490, 492));
    assert_eq!(v4.qqp(), V3(490, 490, 491));
    assert_eq!(v4.qqq(), V3(490, 490, 490));
    assert_eq!(v4.ssss(), V4(489, 489, 489, 489));
    assert_eq!(v4.ssst(), V4(489, 489, 489, 492));
    assert_eq!(v4.sssp(), V4(489, 489, 489, 491));
    assert_eq!(v4.sssq(), V4(489, 489, 489, 490));
    assert_eq!(v4.ssts(), V4(489, 489, 492, 489));
    assert_eq!(v4.sstt(), V4(489, 489, 492, 492));
    assert_eq!(v4.sstp(), V4(489, 489, 492, 491));
    assert_eq!(v4.sstq(), V4(489, 489, 492, 490));
    assert_eq!(v4.ssps(), V4(489, 489, 491, 489));
    assert_eq!(v4.sspt(), V4(489, 489, 491, 492));
    assert_eq!(v4.sspp(), V4(489, 489, 491, 491));
    assert_eq!(v4.sspq(), V4(489, 489, 491, 490));
    assert_eq!(v4.ssqs(), V4(489, 489, 490, 489));
    assert_eq!(v4.ssqt(), V4(489, 489, 490, 492));
    assert_eq!(v4.ssqp(), V4(489, 489, 490, 491));
    assert_eq!(v4.ssqq(), V4(489, 489, 490, 490));
    assert_eq!(v4.stss(), V4(489, 492, 489, 489));
    assert_eq!(v4.stst(), V4(489, 492, 489, 492));
    assert_eq!(v4.stsp(), V4(489, 492, 489, 491));
    assert_eq!(v4.stsq(), V4(489, 492, 489, 490));
    assert_eq!(v4.stts(), V4(489, 492, 492, 489));
    assert_eq!(v4.sttt(), V4(489, 492, 492, 492));
    assert_eq!(v4.sttp(), V4(489, 492, 492, 491));
    assert_eq!(v4.sttq(), V4(489, 492, 492, 490));
    assert_eq!(v4.stps(), V4(489, 492, 491, 489));
    assert_eq!(v4.stpt(), V4(489, 492, 491, 492));
    assert_eq!(v4.stpp(), V4(489, 492, 491, 491));
    v4.set_stpq(V4(493, 494, 495, 496));
    assert_eq!(v4.stpq(), V4(493, 494, 495, 496));
    assert_eq!(v4.stqs(), V4(493, 494, 496, 493));
    assert_eq!(v4.stqt(), V4(493, 494, 496, 494));
    v4.set_stqp(V4(497, 498, 499, 500));
    assert_eq!(v4.stqp(), V4(497, 498, 499, 500));
    assert_eq!(v4.stqq(), V4(497, 498, 499, 499));
    assert_eq!(v4.spss(), V4(497, 500, 497, 497));
    assert_eq!(v4.spst(), V4(497, 500, 497, 498));
    assert_eq!(v4.spsp(), V4(497, 500, 497, 500));
    assert_eq!(v4.spsq(), V4(497, 500, 497, 499));
    assert_eq!(v4.spts(), V4(497, 500, 498, 497));
    assert_eq!(v4.sptt(), V4(497, 500, 498, 498));
    assert_eq!(v4.sptp(), V4(497, 500, 498, 500));
    v4.set_sptq(V4(501, 502, 503, 504));
    assert_eq!(v4.sptq(), V4(501, 502, 503, 504));
    assert_eq!(v4.spps(), V4(501, 502, 502, 501));
    assert_eq!(v4.sppt(), V4(501, 502, 502, 503));
    assert_eq!(v4.sppp(), V4(501, 502, 502, 502));
    assert_eq!(v4.sppq(), V4(501, 502, 502, 504));
    assert_eq!(v4.spqs(), V4(501, 502, 504, 501));
    v4.set_spqt(V4(505, 506, 507, 508));
    assert_eq!(v4.spqt(), V4(505, 506, 507, 508));
    assert_eq!(v4.spqp(), V4(505, 506, 507, 506));
    assert_eq!(v4.spqq(), V4(505, 506, 507, 507));
    assert_eq!(v4.sqss(), V4(505, 507, 505, 505));
    assert_eq!(v4.sqst(), V4(505, 507, 505, 508));
    assert_eq!(v4.sqsp(), V4(505, 507, 505, 506));
    assert_eq!(v4.sqsq(), V4(505, 507, 505, 507));
    assert_eq!(v4.sqts(), V4(505, 507, 508, 505));
    assert_eq!(v4.sqtt(), V4(505, 507, 508, 508));
    v4.set_sqtp(V4(509, 510, 511, 512));
    assert_eq!(v4.sqtp(), V4(509, 510, 511, 512));
    assert_eq!(v4.sqtq(), V4(509, 510, 511, 510));
    assert_eq!(v4.sqps(), V4(509, 510, 512, 509));
    v4.set_sqpt(V4(513, 514, 515, 516));
    assert_eq!(v4.sqpt(), V4(513, 514, 515, 516));
    assert_eq!(v4.sqpp(), V4(513, 514, 515, 515));
    assert_eq!(v4.sqpq(), V4(513, 514, 515, 514));
    assert_eq!(v4.sqqs(), V4(513, 514, 514, 513));
    assert_eq!(v4.sqqt(), V4(513, 514, 514, 516));
    assert_eq!(v4.sqqp(), V4(513, 514, 514, 515));
    assert_eq!(v4.sqqq(), V4(513, 514, 514, 514));
    assert_eq!(v4.tsss(), V4(516, 513, 513, 513));
    assert_eq!(v4.tsst(), V4(516, 513, 513, 516));
    assert_eq!(v4.tssp(), V4(516, 513, 513, 515));
    assert_eq!(v4.tssq(), V4(516, 513, 513, 514));
    assert_eq!(v4.tsts(), V4(516, 513, 516, 513));
    assert_eq!(v4.tstt(), V4(516, 513, 516, 516));
    assert_eq!(v4.tstp(), V4(516, 513, 516, 515));
    assert_eq!(v4.tstq(), V4(516, 513, 516, 514));
    assert_eq!(v4.tsps(), V4(516, 513, 515, 513));
    assert_eq!(v4.tspt(), V4(516, 513, 515, 516));
    assert_eq!(v4.tspp(), V4(516, 513, 515, 515));
    v4.set_tspq(V4(517, 518, 519, 520));
    assert_eq!(v4.tspq(), V4(517, 518, 519, 520));
    assert_eq!(v4.tsqs(), V4(517, 518, 520, 518));
    assert_eq!(v4.tsqt(), V4(517, 518, 520, 517));
    v4.set_tsqp(V4(521, 522, 523, 524));
    assert_eq!(v4.tsqp(), V4(521, 522, 523, 524));
    assert_eq!(v4.tsqq(), V4(521, 522, 523, 523));
    assert_eq!(v4.ttss(), V4(521, 521, 522, 522));
    assert_eq!(v4.ttst(), V4(521, 521, 522, 521));
    assert_eq!(v4.ttsp(), V4(521, 521, 522, 524));
    assert_eq!(v4.ttsq(), V4(521, 521, 522, 523));
    assert_eq!(v4.ttts(), V4(521, 521, 521, 522));
    assert_eq!(v4.tttt(), V4(521, 521, 521, 521));
    assert_eq!(v4.tttp(), V4(521, 521, 521, 524));
    assert_eq!(v4.tttq(), V4(521, 521, 521, 523));
    assert_eq!(v4.ttps(), V4(521, 521, 524, 522));
    assert_eq!(v4.ttpt(), V4(521, 521, 524, 521));
    assert_eq!(v4.ttpp(), V4(521, 521, 524, 524));
    assert_eq!(v4.ttpq(), V4(521, 521, 524, 523));
    assert_eq!(v4.ttqs(), V4(521, 521, 523, 522));
    assert_eq!(v4.ttqt(), V4(521, 521, 523, 521));
    assert_eq!(v4.ttqp(), V4(521, 521, 523, 524));
    assert_eq!(v4.ttqq(), V4(521, 521, 523, 523));
    assert_eq!(v4.tpss(), V4(521, 524, 522, 522));
    assert_eq!(v4.tpst(), V4(521, 524, 522, 521));
    assert_eq!(v4.tpsp(), V4(521, 524, 522, 524));
    v4.set_tpsq(V4(525, 526, 527, 528));
    assert_eq!(v4.tpsq(), V4(525, 526, 527, 528));
    assert_eq!(v4.tpts(), V4(525, 526, 525, 527));
    assert_eq!(v4.tptt(), V4(525, 526, 525, 525));
    assert_eq!(v4.tptp(), V4(525, 526, 525, 526));
    assert_eq!(v4.tptq(), V4(525, 526, 525, 528));
    assert_eq!(v4.tpps(), V4(525, 526, 526, 527));
    assert_eq!(v4.tppt(), V4(525, 526, 526, 525));
    assert_eq!(v4.tppp(), V4(525, 526, 526, 526));
    assert_eq!(v4.tppq(), V4(525, 526, 526, 528));
    v4.set_tpqs(V4(529, 530, 531, 532));
    assert_eq!(v4.tpqs(), V4(529, 530, 531, 532));
    assert_eq!(v4.tpqt(), V4(529, 530, 531, 529));
    assert_eq!(v4.tpqp(), V4(529, 530, 531, 530));
    assert_eq!(v4.tpqq(), V4(529, 530, 531, 531));
    assert_eq!(v4.tqss(), V4(529, 531, 532, 532));
    assert_eq!(v4.tqst(), V4(529, 531, 532, 529));
    v4.set_tqsp(V4(533, 534, 535, 536));
    assert_eq!(v4.tqsp(), V4(533, 534, 535, 536));
    assert_eq!(v4.tqsq(), V4(533, 534, 535, 534));
    assert_eq!(v4.tqts(), V4(533, 534, 533, 535));
    assert_eq!(v4.tqtt(), V4(533, 534, 533, 533));
    assert_eq!(v4.tqtp(), V4(533, 534, 533, 536));
    assert_eq!(v4.tqtq(), V4(533, 534, 533, 534));
    v4.set_tqps(V4(537, 538, 539, 540));
    assert_eq!(v4.tqps(), V4(537, 538, 539, 540));
    assert_eq!(v4.tqpt(), V4(537, 538, 539, 537));
    assert_eq!(v4.tqpp(), V4(537, 538, 539, 539));
    assert_eq!(v4.tqpq(), V4(537, 538, 539, 538));
    assert_eq!(v4.tqqs(), V4(537, 538, 538, 540));
    assert_eq!(v4.tqqt(), V4(537, 538, 538, 537));
    assert_eq!(v4.tqqp(), V4(537, 538, 538, 539));
    assert_eq!(v4.tqqq(), V4(537, 538, 538, 538));
    assert_eq!(v4.psss(), V4(539, 540, 540, 540));
    assert_eq!(v4.psst(), V4(539, 540, 540, 537));
    assert_eq!(v4.pssp(), V4(539, 540, 540, 539));
    assert_eq!(v4.pssq(), V4(539, 540, 540, 538));
    assert_eq!(v4.psts(), V4(539, 540, 537, 540));
    assert_eq!(v4.pstt(), V4(539, 540, 537, 537));
    assert_eq!(v4.pstp(), V4(539, 540, 537, 539));
    v4.set_pstq(V4(541, 542, 543, 544));
    assert_eq!(v4.pstq(), V4(541, 542, 543, 544));
    assert_eq!(v4.psps(), V4(541, 542, 541, 542));
    assert_eq!(v4.pspt(), V4(541, 542, 541, 543));
    assert_eq!(v4.pspp(), V4(541, 542, 541, 541));
    assert_eq!(v4.pspq(), V4(541, 542, 541, 544));
    assert_eq!(v4.psqs(), V4(541, 542, 544, 542));
    v4.set_psqt(V4(545, 546, 547, 548));
    assert_eq!(v4.psqt(), V4(545, 546, 547, 548));
    assert_eq!(v4.psqp(), V4(545, 546, 547, 545));
    assert_eq!(v4.psqq(), V4(545, 546, 547, 547));
    assert_eq!(v4.ptss(), V4(545, 548, 546, 546));
    assert_eq!(v4.ptst(), V4(545, 548, 546, 548));
    assert_eq!(v4.ptsp(), V4(545, 548, 546, 545));
    v4.set_ptsq(V4(549, 550, 551, 552));
    assert_eq!(v4.ptsq(), V4(549, 550, 551, 552));
    assert_eq!(v4.ptts(), V4(549, 550, 550, 551));
    assert_eq!(v4.pttt(), V4(549, 550, 550, 550));
    assert_eq!(v4.pttp(), V4(549, 550, 550, 549));
    assert_eq!(v4.pttq(), V4(549, 550, 550, 552));
    assert_eq!(v4.ptps(), V4(549, 550, 549, 551));
    assert_eq!(v4.ptpt(), V4(549, 550, 549, 550));
    assert_eq!(v4.ptpp(), V4(549, 550, 549, 549));
    assert_eq!(v4.ptpq(), V4(549, 550, 549, 552));
    v4.set_ptqs(V4(553, 554, 555, 556));
    assert_eq!(v4.ptqs(), V4(553, 554, 555, 556));
    assert_eq!(v4.ptqt(), V4(553, 554, 555, 554));
    assert_eq!(v4.ptqp(), V4(553, 554, 555, 553));
    assert_eq!(v4.ptqq(), V4(553, 554, 555, 555));
    assert_eq!(v4.ppss(), V4(553, 553, 556, 556));
    assert_eq!(v4.ppst(), V4(553, 553, 556, 554));
    assert_eq!(v4.ppsp(), V4(553, 553, 556, 553));
    assert_eq!(v4.ppsq(), V4(553, 553, 556, 555));
    assert_eq!(v4.ppts(), V4(553, 553, 554, 556));
    assert_eq!(v4.pptt(), V4(553, 553, 554, 554));
    assert_eq!(v4.pptp(), V4(553, 553, 554, 553));
    assert_eq!(v4.pptq(), V4(553, 553, 554, 555));
    assert_eq!(v4.ppps(), V4(553, 553, 553, 556));
    assert_eq!(v4.pppt(), V4(553, 553, 553, 554));
    assert_eq!(v4.pppp(), V4(553, 553, 553, 553));
    assert_eq!(v4.pppq(), V4(553, 553, 553, 555));
    assert_eq!(v4.ppqs(), V4(553, 553, 555, 556));
    assert_eq!(v4.ppqt(), V4(553, 553, 555, 554));
    assert_eq!(v4.ppqp(), V4(553, 553, 555, 553));
    assert_eq!(v4.ppqq(), V4(553, 553, 555, 555));
    assert_eq!(v4.pqss(), V4(553, 555, 556, 556));
    v4.set_pqst(V4(557, 558, 559, 560));
    assert_eq!(v4.pqst(), V4(557, 558, 559, 560));
    assert_eq!(v4.pqsp(), V4(557, 558, 559, 557));
    assert_eq!(v4.pqsq(), V4(557, 558, 559, 558));
    v4.set_pqts(V4(561, 562, 563, 564));
    assert_eq!(v4.pqts(), V4(561, 562, 563, 564));
    assert_eq!(v4.pqtt(), V4(561, 562, 563, 563));
    assert_eq!(v4.pqtp(), V4(561, 562, 563, 561));
    assert_eq!(v4.pqtq(), V4(561, 562, 563, 562));
    assert_eq!(v4.pqps(), V4(561, 562, 561, 564));
    assert_eq!(v4.pqpt(), V4(561, 562, 561, 563));
    assert_eq!(v4.pqpp(), V4(561, 562, 561, 561));
    assert_eq!(v4.pqpq(), V4(561, 562, 561, 562));
    assert_eq!(v4.pqqs(), V4(561, 562, 562, 564));
    assert_eq!(v4.pqqt(), V4(561, 562, 562, 563));
    assert_eq!(v4.pqqp(), V4(561, 562, 562, 561));
    assert_eq!(v4.pqqq(), V4(561, 562, 562, 562));
    assert_eq!(v4.qsss(), V4(562, 564, 564, 564));
    assert_eq!(v4.qsst(), V4(562, 564, 564, 563));
    assert_eq!(v4.qssp(), V4(562, 564, 564, 561));
    assert_eq!(v4.qssq(), V4(562, 564, 564, 562));
    assert_eq!(v4.qsts(), V4(562, 564, 563, 564));
    assert_eq!(v4.qstt(), V4(562, 564, 563, 563));
    v4.set_qstp(V4(565, 566, 567, 568));
    assert_eq!(v4.qstp(), V4(565, 566, 567, 568));
    assert_eq!(v4.qstq(), V4(565, 566, 567, 565));
    assert_eq!(v4.qsps(), V4(565, 566, 568, 566));
    v4.set_qspt(V4(569, 570, 571, 572));
    assert_eq!(v4.qspt(), V4(569, 570, 571, 572));
    assert_eq!(v4.qspp(), V4(569, 570, 571, 571));
    assert_eq!(v4.qspq(), V4(569, 570, 571, 569));
    assert_eq!(v4.qsqs(), V4(569, 570, 569, 570));
    assert_eq!(v4.qsqt(), V4(569, 570, 569, 572));
    assert_eq!(v4.qsqp(), V4(569, 570, 569, 571));
    assert_eq!(v4.qsqq(), V4(569, 570, 569, 569));
    assert_eq!(v4.qtss(), V4(569, 572, 570, 570));
    assert_eq!(v4.qtst(), V4(569, 572, 570, 572));
    v4.set_qtsp(V4(573, 574, 575, 576));
    assert_eq!(v4.qtsp(), V4(573, 574, 575, 576));
    assert_eq!(v4.qtsq(), V4(573, 574, 575, 573));
    assert_eq!(v4.qtts(), V4(573, 574, 574, 575));
    assert_eq!(v4.qttt(), V4(573, 574, 574, 574));
    assert_eq!(v4.qttp(), V4(573, 574, 574, 576));
    assert_eq!(v4.qttq(), V4(573, 574, 574, 573));
    v4.set_qtps(V4(577, 578, 579, 580));
    assert_eq!(v4.qtps(), V4(577, 578, 579, 580));
    assert_eq!(v4.qtpt(), V4(577, 578, 579, 578));
    assert_eq!(v4.qtpp(), V4(577, 578, 579, 579));
    assert_eq!(v4.qtpq(), V4(577, 578, 579, 577));
    assert_eq!(v4.qtqs(), V4(577, 578, 577, 580));
    assert_eq!(v4.qtqt(), V4(577, 578, 577, 578));
    assert_eq!(v4.qtqp(), V4(577, 578, 577, 579));
    assert_eq!(v4.qtqq(), V4(577, 578, 577, 577));
    assert_eq!(v4.qpss(), V4(577, 579, 580, 580));
    v4.set_qpst(V4(581, 582, 583, 584));
    assert_eq!(v4.qpst(), V4(581, 582, 583, 584));
    assert_eq!(v4.qpsp(), V4(581, 582, 583, 582));
    assert_eq!(v4.qpsq(), V4(581, 582, 583, 581));
    v4.set_qpts(V4(585, 586, 587, 588));
    assert_eq!(v4.qpts(), V4(585, 586, 587, 588));
    assert_eq!(v4.qptt(), V4(585, 586, 587, 587));
    assert_eq!(v4.qptp(), V4(585, 586, 587, 586));
    assert_eq!(v4.qptq(), V4(585, 586, 587, 585));
    assert_eq!(v4.qpps(), V4(585, 586, 586, 588));
    assert_eq!(v4.qppt(), V4(585, 586, 586, 587));
    assert_eq!(v4.qppp(), V4(585, 586, 586, 586));
    assert_eq!(v4.qppq(), V4(585, 586, 586, 585));
    assert_eq!(v4.qpqs(), V4(585, 586, 585, 588));
    assert_eq!(v4.qpqt(), V4(585, 586, 585, 587));
    assert_eq!(v4.qpqp(), V4(585, 586, 585, 586));
    assert_eq!(v4.qpqq(), V4(585, 586, 585, 585));
    assert_eq!(v4.qqss(), V4(585, 585, 588, 588));
    assert_eq!(v4.qqst(), V4(585, 585, 588, 587));
    assert_eq!(v4.qqsp(), V4(585, 585, 588, 586));
    assert_eq!(v4.qqsq(), V4(585, 585, 588, 585));
    assert_eq!(v4.qqts(), V4(585, 585, 587, 588));
    assert_eq!(v4.qqtt(), V4(585, 585, 587, 587));
    assert_eq!(v4.qqtp(), V4(585, 585, 587, 586));
    assert_eq!(v4.qqtq(), V4(585, 585, 587, 585));
    assert_eq!(v4.qqps(), V4(585, 585, 586, 588));
    assert_eq!(v4.qqpt(), V4(585, 585, 586, 587));
    assert_eq!(v4.qqpp(), V4(585, 585, 586, 586));
    assert_eq!(v4.qqpq(), V4(585, 585, 586, 585));
    assert_eq!(v4.qqqs(), V4(585, 585, 585, 588));
    assert_eq!(v4.qqqt(), V4(585, 585, 585, 587));
    assert_eq!(v4.qqqp(), V4(585, 585, 585, 586));
    assert_eq!(v4.qqqq(), V4(585, 585, 585, 585));
  }
}
