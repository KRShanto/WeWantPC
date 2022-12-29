import Link from "next/link";
import React, { useState } from "react";

export default function Register() {
  // name, email, password, password2
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [password2, setPassword2] = useState("");

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();

    if (
      name === "" ||
      email === "" ||
      password === "" ||
      password2 === "" ||
      password !== password2
    ) {
      console.log("Empty fields");
      return;
    }

    const res = await fetch("/api/register", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, email, password }),
    });
    const data = await res.json();

    if (data.type === "SUCCESS") {
      console.log("success");
    }
  }

  return (
    <div>
      <form action="#" onSubmit={handleSubmit}>
        <label htmlFor="name">Name</label>
        <input
          type="text"
          name="name"
          id="name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <label htmlFor="email">Email</label>
        <input
          type="email"
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
        <label htmlFor="password2">Confirm Password</label>
        <input
          type="password"
          name="password2"
          id="password2"
          value={password2}
          onChange={(e) => setPassword2(e.target.value)}
        />

        <p className="already-have-account">
          Already have an account? <Link href="/login">Login</Link>
        </p>

        <button type="submit">Register</button>
      </form>
    </div>
  );
}
