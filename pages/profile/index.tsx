import React from "react";
import { AppProps } from "next/app";
import VNav from "../../components/VNav";

export default function ProfilePage() {
  return (
    <>
      <div className="profile">
        <VNav
          links={[
            { name: "Cart", link: "cart" },
            { name: "My Orders", link: "orders" },
            { name: "Settings", link: "/settings" },
          ]}
        />
      </div>
    </>
  );
}
