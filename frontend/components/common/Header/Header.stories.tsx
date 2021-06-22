import { Meta, Story } from "@storybook/react";

import { Header } from "./Header";

import { AuthUserContextMockProvider } from "../useAuthUser/context";

export default {
  title: "common/Header",
  component: Header,
} as Meta;

export const HeaderLoggedIn: Story = () => <Header />;
HeaderLoggedIn.storyName = "Header Normal";
HeaderLoggedIn.decorators = [
  (storyFn) => (
    <>
      <AuthUserContextMockProvider>{storyFn()}</AuthUserContextMockProvider>
    </>
  ),
];
