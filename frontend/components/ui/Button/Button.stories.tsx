import { Meta, Story } from "@storybook/react";

import { Button } from "./Button";
import { SessionProvider } from "next-auth/react";

export default {
  title: "common/Button",
  component: Button,
} as Meta;

export const ButtonStory: Story = () => <Button />;
ButtonStory.decorators = [
  (storyFn) => <SessionProvider>{storyFn()}</SessionProvider>,
];
