import { useAuthUser } from "../components/common/useAuthUser/useAuthUser";

function HomePage() {
  const { user } = useAuthUser();
  return <div>Welcome to Next.js! {user && user.email}</div>;
}

export default HomePage;
