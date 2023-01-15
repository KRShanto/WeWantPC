import { useContext, useState, useEffect } from "react";
import Link from "next/link";
import { UserContext } from "../contexts/user";

export default function Navbar() {
  const userContext = useContext(UserContext);
  const user = userContext?.user;

  return (
    <nav>
      <div className="logo">WeWantPC</div>

      <div className="search-bar">
        <input type="text" placeholder="Search" />
        <button>Search</button>
      </div>

      <div className="links">
        <Link href="/">Home</Link>
        <Link href="/pc-components">PC Components</Link>
        <Link href="/compare">Compare</Link>
        <Link href="/login">Login</Link>
        <Link href="/cart">Cart</Link>
        <Link href="/help">Help</Link>
        {user && <Link href="/profile">Profile</Link>}
        {user && user.role === "Admin" && <Link href="/admin">Admin</Link>}
      </div>
    </nav>
  );
}
