struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    fn scale(&mut self, sx: f64, sy: f64, sz: f64) {
        self.x *= sx;
        self.y *= sy;
        self.z *= sz;
    }

    fn rotate_x(&mut self, angle: f64) {
        let new_y = self.y * angle.cos() - self.z * angle.sin();
        let new_z = self.y * angle.sin() + self.z * angle.cos();
        self.y = new_y;
        self.z = new_z;
    }

      fn rotate_y(&mut self, angle: f64) {
        let new_x = self.x * angle.cos() + self.z * angle.sin();
        let new_z = -self.x * angle.sin() + self.z * angle.cos();
        self.x = new_x;
        self.z = new_z;
    }

    fn rotate_z(&mut self, angle: f64) {
        let new_x = self.x * angle.cos() - self.y * angle.sin();
        let new_y = self.x * angle.sin() + self.y * angle.cos();
        self.x = new_x;
        self.y = new_y;
    }

    fn reflect_xy_plane(&mut self) {
        self.z = -self.z;
    }

    fn reflect_yz_plane(&mut self) {
        self.x = -self.x;
    }

    fn reflect_xz_plane(&mut self) {
        self.y = -self.y;
    }

    fn shear_xy_plane(&mut self, shx: f64, shy: f64) {
        self.x += self.y * shx;
        self.y += self.x * shy;
    }

    fn shear_yz_plane(&mut self, shy: f64, shz: f64) {
        self.y += self.z * shy;
        self.z += self.y * shz;
    }

    fn shear_xz_plane(&mut self, shx: f64, shz: f64) {
        self.x += self.z * shx;
        self.z += self.x * shz;
    }
}

fn main() {
    // Create a point in 3D space
    let mut point = Point3D::new(1.0, 2.0, 3.0);

    println!(
        "Before translation: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // Translate the point
    point.translate(2.0, 3.0, 4.0);

    println!(
        "After translation: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    point.scale(2.0, 0.5, 3.0);

    // Display coordinates after scaling
    println!("After scaling: x={}, y={}, z={}", point.x, point.y, point.z);

    // Rotate the point around the x-axis by 45 degrees (in radians)
    point.rotate_x(45.0f64.to_radians());

    // Display coordinates after rotation
    println!(
        "After rotation X: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // Reflect the point across the XY-plane
    point.reflect_xy_plane();
    println!(
        "After XY-plane reflection: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // // Reflect the point across the YZ-plane
    point.reflect_yz_plane();
    println!(
        "After YZ-plane reflection: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // Reflect the point across the XZ-plane
    point.reflect_xz_plane();
    println!(
        "After XZ-plane reflection: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // Shear the point along the XY-plane
    point.shear_xy_plane(0.5, 0.0);
    println!(
        "After XY-plane shearing: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // Shear the point along the YZ-plane
    point.shear_yz_plane(0.0, 0.5);
    println!(
        "After YZ-plane shearing: x={}, y={}, z={}",
        point.x, point.y, point.z
    );

    // Shear the point along the XZ-plane
    point.shear_xz_plane(0.0, 0.5);
    println!(
        "After XZ-plane shearing: x={}, y={}, z={}",
        point.x, point.y, point.z
    );
}
