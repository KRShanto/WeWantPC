import { GetServerSidePropsContext } from "next";
import React from "react";
import get from "../../utils/get";
import VNav from "../../components/VNav";

export default function Dashboard() {
  return (
    <>
      <div className="dashboard">
        <VNav
          links={[
            { name: "Dashboard", link: "" },
            { name: "Users", link: "users" },
            { name: "Staff Users", link: "staff" },
            { name: "Admin Users", link: "admins" },
            { name: "Products", link: "products" },
            { name: "Orders", link: "orders" },
            { name: "Categories", link: "categories" },
          ]}
        />
      </div>
    </>
  );
}

export async function getServerSideProps(context: GetServerSidePropsContext) {
  // TODO: Check if user is logged in
  // Also check if user is admin
  // If not, send 404

  // read the cookie
  // const cookie = context.req.cookies;

  // console.log("cookie", cookie);

  // return {
  //   props: {},
  // };

  // const json = await get("/whoami");
  const res = await fetch(`${process.env.NEXT_PUBLIC_SERVER_URL}/whoami`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      // set cookie
      Cookie: context.req.headers.cookie ? context.req.headers.cookie : "",
    },
    credentials: "include",
  });

  // console.log("json", await res.json());

  // return {
  //   props: {},
  // };

  const json = await res.json();

  if (json.type === "Success" && json.data.role === "Admin") {
    return {
      props: {},
    };
  } else {
    return {
      notFound: true,
    };
  }
}
