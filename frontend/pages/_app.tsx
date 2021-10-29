import { AppPropsType } from "next/dist/next-server/lib/utils";
import { VFC } from "react";
import { Header } from "../components/ui/Header/Header";
import { AuthUserContextProvider } from "../hooks/useAuthUser/context";

const App: VFC<{
  Component: VFC<AppPropsType>;
  pageProps: AppPropsType;
}> = function ({ Component, pageProps }) {
  return (
    <>
      <AuthUserContextProvider>
        <>
          <Header />
          <Component {...pageProps} />
        </>
      </AuthUserContextProvider>
    </>
  );
};

export default App;
