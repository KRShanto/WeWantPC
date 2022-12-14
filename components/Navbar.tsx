import { useState, useEffect } from "react";
import Link from "next/link";

export default function Navbar() {
  // home, pc-components, compare, login/register, dashboard(later), cart, help

  return (
    <nav>
      <ul>
        <Link href="/">Home</Link>
        <Link href="/pc-components">PC Components</Link>
        <Link href="/compare">Compare</Link>
        <Link href="/login">Login/Register</Link>
        <Link href="/cart">Cart</Link>
        <Link href="/help">Help</Link>
        {/* TODO <Link href="/dashboard">Dashboard</Link> */}
      </ul>
    </nav>
  );
}
