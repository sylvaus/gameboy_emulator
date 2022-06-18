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
}