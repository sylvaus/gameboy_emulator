#include <memory>

#include <gtest/gtest.h>

#include "../mocks/video/renderer.h"
#include "emulator/video/controller.h"
#include "emulator/utils/ranges.h"


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

    TEST_F(VideoControllerFixture, SetGetLCDWindowPositionX)
    {
        const uint8_t value = 35;
        controller->set_lcd_window_position_x(value);
        ASSERT_EQ(value, controller->get_lcd_window_position_x());
    }
}