import { Result$Error, Result$Ok } from "./gleam.mjs";
import { hrtime } from "node:process";

const origin = hrtime.bigint();

export function environment(name) {
  const value = process.env[name];
  return value === undefined
    ? Result$Error(`Missing environment variable: ${name}`)
    : Result$Ok(value);
}

export function monotonic_ns() {
  return Number(hrtime.bigint() - origin);
}

export function consume(value) {
  globalThis.geamBenchmarkResult = value;
  return value;
}
