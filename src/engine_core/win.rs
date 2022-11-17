use windows::{core::*, Foundation::Numerics::*, Win32::Foundation::*, Win32::Graphics::Direct2D::Common::*, Win32::Graphics::Direct2D::*, Win32::Graphics::Direct3D::*, Win32::Graphics::Direct3D11::*, Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*, Win32::Graphics::Gdi::*, Win32::System::Com::*, Win32::System::LibraryLoader::*, Win32::System::Performance::*, Win32::System::SystemInformation::GetLocalTime, Win32::UI::Animation::*, Win32::UI::WindowsAndMessaging::*};
use crate::shapes::{Cube};
use crate::engine_core::tracking_keyboard_and_mouse::{LastPressKey};

fn create_brush(target: &ID2D1DeviceContext) -> Result<ID2D1SolidColorBrush> {
    let color = D2D1_COLOR_F { r: 0.9, g: 0.8, b: 0.1, a: 1.0 };

    let properties = D2D1_BRUSH_PROPERTIES { opacity: 0.8, transform: Matrix3x2::identity() };

    unsafe { target.CreateSolidColorBrush(&color, &properties) }
}

fn create_factory() -> Result<ID2D1Factory1> {
    let mut options = D2D1_FACTORY_OPTIONS::default();

    if cfg!(debug_assertions) {
        options.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
    }

    unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, &options) }
}

fn create_style(factory: &ID2D1Factory1) -> Result<ID2D1StrokeStyle> {
    let props = D2D1_STROKE_STYLE_PROPERTIES { startCap: D2D1_CAP_STYLE_FLAT, endCap: D2D1_CAP_STYLE_FLAT, ..Default::default() };

    unsafe { factory.CreateStrokeStyle(&props, &[]) }
}

fn create_device_with_type(drive_type: D3D_DRIVER_TYPE) -> Result<ID3D11Device> {
    let mut flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;

    if cfg!(debug_assertions) {
        flags |= D3D11_CREATE_DEVICE_DEBUG;
    }

    let mut device = None;

    unsafe { D3D11CreateDevice(None, drive_type, None, flags, &[], D3D11_SDK_VERSION, &mut device, std::ptr::null_mut(), &mut None).map(|()| device.unwrap()) }
}

fn create_device() -> Result<ID3D11Device> {
    let mut result = create_device_with_type(D3D_DRIVER_TYPE_HARDWARE);

    if let Err(err) = &result {
        if err.code() == DXGI_ERROR_UNSUPPORTED {
            result = create_device_with_type(D3D_DRIVER_TYPE_WARP);
        }
    }

    result
}

fn create_render_target(factory: &ID2D1Factory1, device: &ID3D11Device) -> Result<ID2D1DeviceContext> {
    unsafe {
        let d2device = factory.CreateDevice(&device.cast::<IDXGIDevice>()?)?;

        let target = d2device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?;

        target.SetUnitMode(D2D1_UNIT_MODE_DIPS);

        Ok(target)
    }
}

fn get_dxgi_factory(device: &ID3D11Device) -> Result<IDXGIFactory2> {
    let dxdevice = device.cast::<IDXGIDevice>()?;
    unsafe { dxdevice.GetAdapter()?.GetParent() }
}

fn create_swapchain_bitmap(swapchain: &IDXGISwapChain1, target: &ID2D1DeviceContext) -> Result<()> {
    let surface: IDXGISurface = unsafe { swapchain.GetBuffer(0)? };

    let props = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_IGNORE },
        dpiX: 96.0,
        dpiY: 96.0,
        bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
        colorContext: None,
    };

    unsafe {
        let bitmap = target.CreateBitmapFromDxgiSurface(&surface, &props)?;
        target.SetTarget(&bitmap);
    };

    Ok(())
}

fn create_swapchain(device: &ID3D11Device, window: HWND) -> Result<IDXGISwapChain1> {
    let factory = get_dxgi_factory(device)?;

    let props = DXGI_SWAP_CHAIN_DESC1 {
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        ..Default::default()
    };

    unsafe { factory.CreateSwapChainForHwnd(device, window, &props, std::ptr::null(), None) }
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}

pub struct Window {
    handle: HWND,
    factory: ID2D1Factory1,
    dxfactory: IDXGIFactory2,
    pub style: ID2D1StrokeStyle,

    pub target: Option<ID2D1DeviceContext>,
    swapchain: Option<IDXGISwapChain1>,
    pub brush: Option<ID2D1SolidColorBrush>,
    draw_space: Option<ID2D1Bitmap1>,
    dpi: f32,
    visible: bool,
    occlusion: u32,
    cubes: Vec<Cube>,
    pub last_press_key: LastPressKey
}

impl Window {
    pub fn new() -> Result<Self> {
        let factory = create_factory()?;
        let dxfactory: IDXGIFactory2 = unsafe { CreateDXGIFactory1()? };
        let style = create_style(&factory)?;

        let mut dpi = 0.0;
        let mut dpiy = 0.0;
        unsafe { factory.GetDesktopDpi(&mut dpi, &mut dpiy) };

        Ok(Window {
            handle: HWND(0),
            factory,
            dxfactory,
            style,
            target: None,
            swapchain: None,
            brush: None,
            draw_space: None,
            dpi,
            visible: false,
            occlusion: 0,
            cubes: Vec::new(),
            last_press_key: LastPressKey(-1000)
        })
    }

    fn render(&mut self) -> Result<()> {
        if self.target.is_none() {
            let device = create_device()?;
            let target = create_render_target(&self.factory, &device)?;
            unsafe { target.SetDpi(self.dpi, self.dpi) };

            let swapchain = create_swapchain(&device, self.handle)?;
            create_swapchain_bitmap(&swapchain, &target)?;

            self.brush = create_brush(&target).ok();
            self.target = Some(target);
            self.swapchain = Some(swapchain);
            self.create_device_size_resources()?;
        }

        let target = self.target.as_ref().unwrap();
        unsafe { target.BeginDraw() };
        self.draw(target)?;

        unsafe {
            target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;
        }

        if let Err(error) = self.present(1, 0) {
            if error.code() == DXGI_STATUS_OCCLUDED {
                self.occlusion = unsafe { self.dxfactory.RegisterOcclusionStatusWindow(self.handle, WM_USER)? };
                self.visible = false;
            } else {
                self.release_device();
            }
        }

        Ok(())
    }

    fn release_device(&mut self) {
        self.target = None;
        self.swapchain = None;
        self.release_device_resources();
    }

    fn release_device_resources(&mut self) {
        self.brush = None;
        self.draw_space = None;
    }

    fn present(&self, sync: u32, flags: u32) -> Result<()> {
        unsafe { self.swapchain.as_ref().unwrap().Present(sync, flags).ok() }
    }

    fn draw(&self, target: &ID2D1DeviceContext) -> Result<()> {
        let draw_space = self.draw_space.as_ref().unwrap();

        unsafe {

            target.Clear(&D2D1_COLOR_F { r: 0.1, g: 0.1, b: 0.1, a: 1.0 });

            let mut previous = None;
            target.GetTarget(&mut previous);
            target.SetTarget(draw_space);
            target.Clear(std::ptr::null());
            self.draw_elements()?;
            target.SetTarget(previous.as_ref());
            target.SetTransform(&Matrix3x2::translation(5.0, 5.0));

            target.SetTransform(&Matrix3x2::identity());

            target.DrawImage(draw_space, std::ptr::null(), std::ptr::null(), D2D1_INTERPOLATION_MODE_LINEAR, D2D1_COMPOSITE_MODE_SOURCE_OVER);
        }

        Ok(())
    }

    fn draw_elements(&self) -> Result<()> {
        self.cubes.iter().for_each(|cube| {
            cube.draw_cube(&self);
        });
        
        Ok(())
    }

    fn create_device_size_resources(&mut self) -> Result<()> {
        let target = self.target.as_ref().unwrap();
        let draw_space = self.create_space(target)?;
        self.draw_space = Some(draw_space);

        Ok(())
    }

    fn create_space(&self, target: &ID2D1DeviceContext) -> Result<ID2D1Bitmap1> {
        let size_f = unsafe { target.GetSize() };

        let size_u = D2D_SIZE_U { width: (size_f.width * self.dpi / 96.0) as u32, height: (size_f.height * self.dpi / 96.0) as u32 };

        let properties = D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: D2D1_PIXEL_FORMAT { format: DXGI_FORMAT_B8G8R8A8_UNORM, alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED },
            dpiX: self.dpi,
            dpiY: self.dpi,
            bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET,
            colorContext: None,
        };

        unsafe { target.CreateBitmap2(size_u, std::ptr::null(), 0, &properties) }
    }

    fn resize_swapchain_bitmap(&mut self) -> Result<()> {
        if let Some(target) = &self.target {
            let swapchain = self.swapchain.as_ref().unwrap();
            unsafe { target.SetTarget(None) };

            if unsafe { swapchain.ResizeBuffers(0, 0, 0, DXGI_FORMAT_UNKNOWN, 0).is_ok() } {
                create_swapchain_bitmap(swapchain, target)?;
                self.create_device_size_resources()?;
            } else {
                self.release_device();
            }

            self.render()?;
        }

        Ok(())
    }

    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    let mut ps = PAINTSTRUCT::default();
                    BeginPaint(self.handle, &mut ps);
                    self.render().unwrap();
                    EndPaint(self.handle, &ps);
                    LRESULT(0)
                }
                WM_SIZE => {
                    if wparam.0 != SIZE_MINIMIZED as usize {
                        self.resize_swapchain_bitmap().unwrap();
                    }
                    LRESULT(0)
                }
                WM_DISPLAYCHANGE => {
                    self.render().unwrap();
                    LRESULT(0)
                }
                WM_USER => {
                    if self.present(0, DXGI_PRESENT_TEST).is_ok() {
                        self.dxfactory.UnregisterOcclusionStatus(self.occlusion);
                        self.occlusion = 0;
                        self.visible = true;
                    }
                    LRESULT(0)
                }
                WM_ACTIVATE => {
                    self.visible = true; // TODO: unpack !HIWORD(wparam);
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.handle, message, wparam, lparam),
            }
        }
    }

    pub fn run(&mut self, cubes: Vec<Cube>) -> Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            debug_assert!(instance.0 != 0);
            let window_class = s!("window");

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_HAND)?,
                hInstance: instance,
                lpszClassName: window_class,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(WINDOW_EX_STYLE::default(), window_class, s!("Sample Window"), WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, None, None, instance, self as *mut _ as _);

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
            let mut message = MSG::default();

            // add cubes
            self.cubes = cubes;

            loop {
                if self.visible {
                    if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(87) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(38)
                    == (1 | -32767 | -32768)
                    {
                        //Up

                        for cube in &mut self.cubes {
                            cube.middle_dot_y += 20.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(83) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(40)
                        == (1 | -32767 | -32768)
                    {
                        //Down

                        for cube in &mut self.cubes {
                            cube.middle_dot_y -= 20.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(65) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(37)
                        == (1 | -32767 | -32768)
                    {
                        //Left

                        for cube in &mut self.cubes {
                            cube.middle_dot_x -= 20.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(68) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(39)
                        == (1 | -32767 | -32768)
                    {
                        //Right

                        for cube in &mut self.cubes {
                            cube.middle_dot_x += 20.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(13)
                        == (1 | -32767 | -32768)
                    {
                        //Enter

                        for cube in &mut self.cubes {
                            cube.middle_dot_z += 20.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(32) | windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(27)
                        == (1 | -32767 | -32768)
                    {
                        //Space

                        for cube in &mut self.cubes {
                            cube.middle_dot_z -= 20.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(107) == (1 | -32767 | -32768) {
                        for cube in &mut self.cubes {
                            cube.size += 1.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(109) == (1 | -32767 | -32768) {
                        for cube in &mut self.cubes {
                            cube.size -= 1.0;
                            cube.builded_cube.is_builded = false;
                        }
                    } else if windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(80) == (1 | -32767 | -32768) {
                        for cube in &mut self.cubes {
                            cube.rotation.is_need_rotate = !cube.rotation.is_need_rotate;
                        }
                    }
                    

                    for cube in &mut self.cubes {
                        cube.build_shape();
                    }

                    self.render()?;

                    while PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
                    GetMessageA(&mut message, None, 0, 0);

                    if message.message == WM_QUIT {
                        return Ok(());
                    }

                    DispatchMessageA(&message);
                }
            }
        }
    }

    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLong(window, GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLong(window, GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }
}