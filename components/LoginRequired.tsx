import React from "react";
import Link from "next/link";

export default function LoginRequired() {
  return (
    <div id="login-reuired">
      <h1>Sorry, you need to be logged in to view this page.</h1>
      <Link href="/login">
        <a>Click here to login</a>
      </Link>
    </div>
  );
}
