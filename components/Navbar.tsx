import { useState, useEffect } from "react";
import Link from "next/link";

export default function Navbar() {
  // home, pc-components, compare, login/register, dashboard(later), cart, help

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
        {/* TODO <Link href="/dashboard">Dashboard</Link> */}
      </div>
    </nav>
  );
}
