use cgmath::{EuclideanSpace, Transform};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub enum Camera {
    FPS(FPSCamera),
    Orbital(OrbitalCamera),
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        match self {
            Camera::FPS(cam) => cam.build_view_projection_matrix(),
            Camera::Orbital(cam) => cam.build_view_projection_matrix(),
        }
    }
}

pub struct OrbitalCamera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

pub struct FPSCamera {
    pub eye: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub yaw: f32,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl FPSCamera {
    pub fn default(aspect_width: u32, aspect_height: u32) -> Self {
        Self {
            eye: (0.0, 1.0, 2.0).into(),
            up: cgmath::Vector3::unit_y(),
            yaw: 0.0,
            aspect: aspect_width as f32 / aspect_height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn target(&self) -> cgmath::Point3<f32> {
        let dir = cgmath::Vector3::unit_z();
        let rot = cgmath::Matrix4::from_angle_y(cgmath::Rad(self.yaw));
        let target: cgmath::Vector3<f32> = rot.transform_vector(dir) + self.eye.to_vec();
        (target.x, target.y, target.z).into()
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target(), self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

impl OrbitalCamera {
    pub fn default(aspect_width: u32, aspect_height: u32) -> Self {
        Self {
            // position the camera 1 unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: aspect_width as f32 / aspect_height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        }
    }

    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    } 
}
