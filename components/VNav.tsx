import React from "react";
import Link from "next/link";
import { useRouter } from "next/router";

export default function VNav({
  links,
  id,
}: {
  links: { name: string; link: string }[];
  id?: string;
}) {
  const router = useRouter();

  return (
    <>
      <div className="vnav" id={id}>
        <div className="vnav-thumb"></div>
        <div className="links">
          {links.map((link, index) =>
            link.link.startsWith("/") ? (
              <Link key={index} className="link" href={link.link}>
                {link.name}
              </Link>
            ) : (
              <Link
                key={index}
                className="link"
                href={router.pathname + "/" + link.link}
              >
                {link.name}
              </Link>
            )
          )}
        </div>
      </div>
    </>
  );
}
