import { createContext } from "react";
import React from "react";

export type UserType = {
  id: string;
  name: string;
  email: string;
  role: string;
  created_at: string;
  address: string;
  phone: string;
  verified: boolean;
  img_url: string;
};

export type UserContextType = {
  user: UserType | null;
  setUser: React.Dispatch<React.SetStateAction<UserType | null>>;
};

export const UserContext = createContext<UserContextType | null>(null);
