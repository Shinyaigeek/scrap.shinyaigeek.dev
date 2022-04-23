import { Meta, Story } from "@storybook/react";

import { Button } from "./Button";

export default {
  title: "common/Button",
  component: Button,
} as Meta;

export const ButtonStory: Story = () => (
  <Button>
    <h1>hoge</h1>
  </Button>
);
