#include <memory>

#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include "../mocks/video/renderer.h"
#include "emulator/video/controller.h"
#include "emulator/utils/ranges.h"

using ::testing::_;
using namespace emulator::video;

namespace
{

    class VideoControllerFixture: public testing::Test
    {
    protected:
        void SetUp() override
        {
            renderer = std::make_shared<MockRenderer>();
            controller = std::make_shared<VideoControllerImpl>(renderer);

        }
        void TearDown() override
        {

        }

        std::shared_ptr<MockRenderer> renderer;
        std::shared_ptr<VideoControllerImpl> controller;
    };

    TEST_F(VideoControllerFixture, SetGetVram)
    {
        emulator::utils::for_range<int>(
                emulator::memory::START_VRAM,
                emulator::memory::END_VRAM + 1,
                [this](const int address){
                    const uint8_t value = address % 217;
                    controller->set_vram(address, value);
                    ASSERT_EQ(value, controller->get_vram(address));
                }
        );
    }

    TEST_F(VideoControllerFixture, SetGetOAM)
    {
        emulator::utils::for_range<int>(
                emulator::memory::START_OAM,
                emulator::memory::END_OAM + 1,
                [this](const int address){
                    const uint8_t value = address % 211;
                    controller->set_oam(address, value);
                    ASSERT_EQ(value, controller->get_oam(address));
                }
        );
    }

    TEST_F(VideoControllerFixture, SetGetLCDControl)
    {
        const uint8_t value = 96;
        controller->set_lcd_control(value);
        ASSERT_EQ(value, controller->get_lcd_control());
    }

    TEST_F(VideoControllerFixture, SetGetLCDStatus)
    {
        const uint8_t value = 43;
        controller->set_lcd_status(value);
        ASSERT_EQ(value, controller->get_lcd_status());
    }

    TEST_F(VideoControllerFixture, SetGetLCDScrollY)
    {
        const uint8_t value = 19;
        controller->set_lcd_scroll_y(value);
        ASSERT_EQ(value, controller->get_lcd_scroll_y());
    }

    TEST_F(VideoControllerFixture, SetGetLCDScrollX)
    {
        const uint8_t value = 45;
        controller->set_lcd_scroll_x(value);
        ASSERT_EQ(value, controller->get_lcd_scroll_x());
    }

    TEST_F(VideoControllerFixture, SetGetLCDCoordinateY)
    {
        const uint8_t value = 68;
        controller->set_lcd_coordinate_y(value);
        ASSERT_EQ(value, controller->get_lcd_coordinate_y());
    }

    TEST_F(VideoControllerFixture, SetGetLCDCompareY)
    {
        const uint8_t value = 74;
        controller->set_lcd_compare_y(value);
        ASSERT_EQ(value, controller->get_lcd_compare_y());
    }

    TEST_F(VideoControllerFixture, SetGetBGPaletteData)
    {
        const uint8_t value = 58;
        controller->set_bg_palette_data(value);
        ASSERT_EQ(value, controller->get_bg_palette_data());
    }

    TEST_F(VideoControllerFixture, SetGetOBJPaletteData0)
    {
        const uint8_t value = 82;
        controller->set_obj_palette_data_0(value);
        ASSERT_EQ(value, controller->get_obj_palette_data_0());
    }

    TEST_F(VideoControllerFixture, SetGetOBJPaletteData1)
    {
        const uint8_t value = 91;
        controller->set_obj_palette_data_1(value);
        ASSERT_EQ(value, controller->get_obj_palette_data_1());
    }

    TEST_F(VideoControllerFixture, SetGetLCDWindowPositionY)
    {
        const uint8_t value = 124;
        controller->set_lcd_window_position_y(value);
        ASSERT_EQ(value, controller->get_lcd_window_position_y());
    }

    TEST_F(VideoControllerFixture, SetGetLCDWindowPositionX)
    {
        const uint8_t value = 35;
        controller->set_lcd_window_position_x(value);
        ASSERT_EQ(value, controller->get_lcd_window_position_x());
    }

    TEST_F(VideoControllerFixture, EnableLCD)
    {
        LcdControl control{};
        control.enable_lcd = false;
        LcdStatus status{};
        status.mode = MODE_0_HBLANK;

        controller->set_lcd_control(control.value);
        controller->set_lcd_status(status.value);
        controller->set_lcd_coordinate_y(15);

        control.enable_lcd = true;
        controller->set_lcd_control(control.value);
        ASSERT_EQ(0, controller->get_lcd_coordinate_y());
        ASSERT_EQ(MODE_0_HBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
    }

    TEST_F(VideoControllerFixture, EnableLCDWhenAlreadyEnableShouldDoNothing)
    {
        LcdControl control{};
        control.enable_lcd = true;
        LcdStatus status{};
        status.mode = MODE_1_VBLANK;

        controller->set_lcd_control(control.value);
        controller->set_lcd_status(status.value);
        controller->set_lcd_coordinate_y(56);

        control.enable_lcd = true;
        controller->set_lcd_control(control.value);
        ASSERT_EQ(56, controller->get_lcd_coordinate_y());
        ASSERT_EQ(MODE_1_VBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
    }

    TEST_F(VideoControllerFixture, DisableLCD)
    {
        LcdControl control{};
        control.enable_lcd = true;
        LcdStatus status{};
        status.mode = MODE_1_VBLANK;

        controller->set_lcd_control(control.value);
        controller->set_lcd_status(status.value);
        controller->set_lcd_coordinate_y(15);

        control.enable_lcd = false;
        controller->set_lcd_control(control.value);
        ASSERT_EQ(0, controller->get_lcd_coordinate_y());
        ASSERT_EQ(MODE_0_HBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
    }

    TEST_F(VideoControllerFixture, TicksCycles)
    {
        // https://gbdev.io/pandocs/STAT.html
        //        Mode 2  2_____2_____2_____2_____2_____2___________________2____
        //        Mode 3  _33____33____33____33____33____33__________________3___
        //        Mode 0  ___000___000___000___000___000___000________________000
        //        Mode 1  ____________________________________11111111111111_____

        LcdControl control{};
        control.enable_lcd = true;
        controller->set_lcd_control(control.value);

        controller->tick(1);
        EXPECT_CALL(*renderer, render_line(_, _, _)).Times(144);
        emulator::utils::for_range<int>(0, 144, [this](const int line) {
            ASSERT_EQ(MODE_2_SEARCH_OAM, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            // Nothing should change if tick time not reached
            controller->tick(MODE_2_TICKS - 30);
            ASSERT_EQ(MODE_2_SEARCH_OAM, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            controller->tick(30);
            ASSERT_EQ(MODE_3_TRANSFER, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            // Nothing should change if tick time not reached
            controller->tick(MODE_3_TICKS - 25);
            ASSERT_EQ(MODE_3_TRANSFER, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            controller->tick(25);
            ASSERT_EQ(MODE_0_HBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            controller->tick(MODE_0_TICKS - 10);
            ASSERT_EQ(MODE_0_HBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());
            controller->tick(10);
        });

        emulator::utils::for_range<int>(144, 154, [this](const int line) {
            ASSERT_EQ(MODE_1_VBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            // Nothing should change if tick time not reached
            controller->tick(MODE_1_TICKS - 15);
            ASSERT_EQ(MODE_1_VBLANK, LcdStatus{.value = controller->get_lcd_status()}.mode);
            ASSERT_EQ(line, controller->get_lcd_coordinate_y());

            controller->tick(15);
        });

        ASSERT_EQ(MODE_2_SEARCH_OAM, LcdStatus{.value = controller->get_lcd_status()}.mode);
        ASSERT_EQ(0, controller->get_lcd_coordinate_y());
    }
}