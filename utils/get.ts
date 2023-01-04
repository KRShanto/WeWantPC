export default async function get(url: string) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_SERVER_URL}${url}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
  });

  return res.json();
}
