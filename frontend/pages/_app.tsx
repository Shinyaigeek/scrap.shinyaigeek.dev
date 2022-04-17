import { AppPropsType } from "next/dist/next-server/lib/utils";
import { VFC } from "react";
import { Header } from "../components/ui/Header/Header";
import { SessionProvider } from "next-auth/react";

const App: VFC<{
  Component: VFC<AppPropsType>;
  pageProps: AppPropsType;
}> = function ({ Component, pageProps: { session, ...pageProps } }) {
  return (
    <SessionProvider session={session}>
      <Header />
      <Component {...pageProps} />
    </SessionProvider>
  );
};

export default App;
