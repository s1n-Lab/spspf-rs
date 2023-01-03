use core::ffi::c_void;

use psp::{
    sys::{
        sceDisplayWaitVblankStart, sceGuAlphaFunc, sceGuBlendFunc, sceGuClear, sceGuClearColor,
        sceGuClearDepth, sceGuDebugFlush, sceGuDebugPrint, sceGuDepthBuffer, sceGuDepthRange,
        sceGuDispBuffer, sceGuDisplay, sceGuDrawBuffer, sceGuEnable, sceGuFinish, sceGuInit,
        sceGuOffset, sceGuScissor, sceGuShadeModel, sceGuStart, sceGuSwapBuffers, sceGuSync,
        sceGuTerm, sceGuTexFunc, sceGuTexMode, sceGuViewport, sceGumLoadIdentity, sceGumMatrixMode,
        sceGumOrtho, sceKernelExitGame, AlphaFunc, BlendFactor, BlendOp, ClearBuffer,
        DisplayPixelFormat, GuContextType, GuState, GuSyncBehavior, GuSyncMode, MatrixMode,
        ShadingModel, TextureColorComponent, TextureEffect, TexturePixelFormat,
    },
    vram_alloc::get_vram_allocator,
    BUF_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use crate::colors::{Color, Colors};

static mut LIST: psp::Align16<[u32; 0x40000]> = psp::Align16([0; 0x40000]);

/// This modules describes and gives access to the PSP screen (width: `480`, height: `272`).
pub struct Canvas {}

impl Canvas {
    /// This method must be called only once and at the start of the project, in the `psp_main` function.
    /// It initiates the graphical functions of the PSP.
    pub fn new() -> Self {
        psp::enable_home_button();

        let allocator = get_vram_allocator().unwrap();
        let fbp0 = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
            .as_mut_ptr_from_zero();
        let fbp1 = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888)
            .as_mut_ptr_from_zero();
        let zbp = allocator
            .alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444)
            .as_mut_ptr_from_zero();

        unsafe {
            sceGumLoadIdentity();
            sceGuInit();

            sceGuStart(GuContextType::Direct, &mut LIST as *mut _ as *mut c_void);
            sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
            sceGuDispBuffer(
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                fbp1 as _,
                BUF_WIDTH as i32,
            );
            sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);

            sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
            sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sceGuDepthRange(65535, 0);
            sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
            sceGuEnable(GuState::ScissorTest);

            sceGuEnable(GuState::Texture2D);
            sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
            sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgb);
            sceGuEnable(GuState::Blend);
            sceGuBlendFunc(
                BlendOp::Add,
                BlendFactor::SrcAlpha,
                BlendFactor::OneMinusSrcAlpha,
                0,
                0,
            );
            sceGuAlphaFunc(AlphaFunc::Greater, 0, 0xff);
            sceGuEnable(GuState::AlphaTest);
            sceGuShadeModel(ShadingModel::Smooth);

            sceGuFinish();
            sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);

            sceDisplayWaitVblankStart();
            sceGuDisplay(true);

            sceGumMatrixMode(MatrixMode::Projection);
            sceGumLoadIdentity();
            sceGumOrtho(0.0, 480.0, 272.0, 0.0, -10.0, 10.0);

            sceGumMatrixMode(MatrixMode::View);
            sceGumLoadIdentity();

            sceGumMatrixMode(MatrixMode::Model);
            sceGumLoadIdentity();
        }

        Canvas {}
    }

    /// Must be called at the start of each frame to prepare the screen for drawing.
    pub fn start_frame(&mut self) {
        unsafe {
            sceGuStart(GuContextType::Direct, &mut LIST as *mut _ as *mut c_void);
        }
    }

    /// Must be called at the end of each frame to actually display the frame to screen.
    pub fn end_frame(&mut self) {
        unsafe {
            sceGuFinish();
            sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
            sceDisplayWaitVblankStart();
            sceGuSwapBuffers();
        }
    }

    /// Clears the screen to a specified color.
    pub fn clear(&mut self, color: Color) {
        unsafe {
            sceGuClearColor(color.as_abgr());
            sceGuClearDepth(0);
            sceGuClear(
                ClearBuffer::COLOR_BUFFER_BIT
                    | ClearBuffer::DEPTH_BUFFER_BIT
                    | ClearBuffer::STENCIL_BUFFER_BIT,
            );
        }
    }

    /// Terminates the script, must only be called once at the end of the project.
    pub fn terminate(&mut self) {
        unsafe {
            sceGuTerm();
            sceKernelExitGame();
        }
    }

    /// Prints a debug text to screen.
    pub fn debug_print(&mut self, text: *const u8) {
        unsafe {
            sceGuDebugPrint(10, 10, Colors::WHITE.as_color().as_abgr(), text);
            sceGuDebugFlush();
        }
    }
}
