import { AppPropsType } from "next/dist/next-server/lib/utils";
import { VFC } from "react";
import { Header } from "../components/common/Header/Header";

const App: VFC<{
    Component:VFC<AppPropsType>,
    pageProps: AppPropsType 
}> = function ({ Component, pageProps }) {
  return (
    <>
      <Header />
      <Component {...pageProps} />
    </>
  );
};

export default App