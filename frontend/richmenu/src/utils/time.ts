
export function formatYmdHi(d: Date): string {
  return `${d.getFullYear()}-${d.getMonth() + 1}-${d.getDate()} ${d.getHours()}:${d.getMinutes()}`
}

export function tomorrow(d: Date): Date {
  return (new Date(d.getFullYear(), d.getMonth(), d.getDate() + 1))
}

