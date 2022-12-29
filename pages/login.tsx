import Link from "next/link";
import React from "react";

export default function Login() {
  async function handleSubmit(e: any) {
    e.preventDefault();
  }

  return (
    <>
      <div className="login-form">
        <h1 className="header">Login</h1>

        <form onSubmit={handleSubmit}>
          <label htmlFor="email">Email</label>
          <input type="email" name="email" id="email" />

          <label htmlFor="password">Password</label>
          <input type="password" name="password" id="password" />

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
