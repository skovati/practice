import { hasDuplicate } from "./duplicate_integer.ts";
import { assertEquals } from "jsr:@std/assert@1";

Deno.test("one", () => {
    assertEquals(hasDuplicate([1, 2, 3, 3]), true)
})

Deno.test("two", () => {
    assertEquals(hasDuplicate([1, 2, 3, 4]), false)
})
