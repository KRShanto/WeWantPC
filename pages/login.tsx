import Link from "next/link";
import React, { useState } from "react";
import post from "../utils/post";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  async function handleSubmit(e: any) {
    e.preventDefault();

    const json = await post("/login", {
      email,
      password,
    });

    if (json.type === "Success") {
      alert("Logged in successfully");
    } else {
      alert("Failed to log in");
    }
  }

  return (
    <>
      <div className="login-form">
        <h1 className="header">Login</h1>

        <form onSubmit={handleSubmit}>
          <label htmlFor="email">Email</label>
          <input
            // type="email"
            type="text"
            name="email"
            id="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />

          <label htmlFor="password">Password</label>
          <input
            type="password"
            name="password"
            id="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />

          <p className="dont-have-account">
            Don&rsquo;t have an account? <Link href="/register">Register</Link>
          </p>

          <p className="forgot-password">
            Forgot your password? <Link href="/reset">Reset</Link>
          </p>
          <button type="submit">Login</button>
        </form>
      </div>
    </>
  );
}
