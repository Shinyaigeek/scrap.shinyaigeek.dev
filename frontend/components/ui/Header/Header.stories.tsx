import { Meta, Story } from "@storybook/react";

import { Header } from "./Header";
import { SessionProvider } from 'next-auth/react';

export default {
  title: "common/Header",
  component: Header,
} as Meta;

export const HeaderLoggedIn: Story = () => <Header />;
HeaderLoggedIn.storyName = "Header Normal";
HeaderLoggedIn.decorators = [(storyFn) => <SessionProvider>{storyFn()}</SessionProvider>];
