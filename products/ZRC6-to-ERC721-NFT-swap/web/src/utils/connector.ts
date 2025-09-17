export function hasInjectedProvider({ flag }: { flag: string }): boolean {
  if (typeof window === "undefined") return false

  const provider = window.ethereum
  return !!(provider && (provider as Record<string, unknown>)[flag])
}

export function getInjectedConnector({ flag }: { flag: string }) {
  // For RainbowKit v2, we'll return a simple connector factory
  return () => {
    if (hasInjectedProvider({ flag })) {
      return window.ethereum
    }
    return null
  }
}
