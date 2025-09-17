import React, { createContext, useContext } from "react"

export function createContainer<T>(useHook: () => T) {
  const Context = createContext<T | null>(null)

  function Provider({ children }: { children: React.ReactNode }) {
    const value = useHook()
    return React.createElement(Context.Provider, { value }, children)
  }

  function useContainer(): T {
    const context = useContext(Context)
    if (!context) {
      throw new Error("useContainer must be used within Provider")
    }
    return context
  }

  return { Provider, useContainer }
}
