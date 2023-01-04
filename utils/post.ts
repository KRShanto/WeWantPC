// Make a post request to the server
export default async function post(url: string, body?: any) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_SERVER_URL}${url}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    credentials: "include",
    body: JSON.stringify(body),
  });

  return res.json();
}
