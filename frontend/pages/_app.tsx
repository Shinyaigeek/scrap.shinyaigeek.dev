import { AppPropsType } from "next/dist/next-server/lib/utils";
import { VFC } from "react";
import { Header } from "../components/ui/Header/Header";
import { SessionProvider } from "next-auth/react";

import "../styles/globals.css";

const App: VFC<{
  Component: VFC<AppPropsType>;
  pageProps: AppPropsType;
}> = function ({ Component, pageProps: { session, ...pageProps } }) {
  return (
    <SessionProvider session={session}>
      <Header />
      <div className="bg-slate-300 min-h-screen">
        <Component {...pageProps} />
      </div>
    </SessionProvider>
  );
};

export default App;
