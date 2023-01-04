import { useEffect, useState } from "react";
import "../styles/globals.scss";
import { useRouter } from "next/router";
import { UserContext } from "../contexts/user";
import { UserType } from "../contexts/user";
import { IsLoadingContext } from "../contexts/isLoading";
import Navbar from "../components/Navbar";
import LoadingBar from "react-top-loading-bar";
import { AppProps } from "next/app";
import Footer from "../components/Footer";
import LoginRequired from "../components/LoginRequired";
import get from "../utils/get";
import { RingLoader } from "react-spinners";

export default function App({ Component, pageProps }: AppProps) {
  const router = useRouter();
  const [progress, setProgress] = useState(0);
  const [user, setUser] = useState<UserType | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const { loginRequired } = pageProps;

  // Check if the user is logged in
  useEffect(() => {
    const fetchUser = async () => {
      // Turn on loading
      setIsLoading(true);

      const json = await get("/whoami");

      if (json.type === "Success") {
        setUser(json.data);
      }

      setIsLoading(false);
    };

    if (!user) {
      fetchUser();
    }
  }, [user]);

  // Set progress bar
  useEffect(() => {
    const handleStart = (url: string) => {
      setProgress(30);
    };
    const handleComplete = (url: string) => {
      setProgress(100);
    };

    router.events.on("routeChangeStart", handleStart);
    router.events.on("routeChangeComplete", handleComplete);
    router.events.on("routeChangeError", handleComplete);

    return () => {
      router.events.off("routeChangeStart", handleStart);
      router.events.off("routeChangeComplete", handleComplete);
      router.events.off("routeChangeError", handleComplete);
    };
  }, [router]);

  return (
    <>
      <UserContext.Provider value={{ user, setUser }}>
        <IsLoadingContext.Provider value={{ isLoading, setIsLoading }}>
          <LoadingBar
            color="#f11946"
            progress={progress}
            height={3}
            onLoaderFinished={() => setProgress(0)}
          />
          <Navbar />

          {isLoading && (
            <div className="spinner-div">
              <RingLoader
                className="spinner"
                color="cyan"
                loading={isLoading}
                size={200}
              />
            </div>
          )}

          <main
            style={{
              opacity: isLoading ? 0.2 : 1,
            }}
          >
            {loginRequired && !isLoading && !user ? (
              <LoginRequired />
            ) : (
              <Component {...pageProps} />
            )}
          </main>

          <Footer />
        </IsLoadingContext.Provider>
      </UserContext.Provider>
    </>
  );
}
