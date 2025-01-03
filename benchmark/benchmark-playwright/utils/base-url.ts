const app = {
  baseUrl: {
    angular: "http://localhost:3000",
    leptos: "http://localhost:3001",
    react: "http://localhost:3002",
    vue: "http://localhost:3003",
  },
};

export const baseUrl = (path: string) => path;
