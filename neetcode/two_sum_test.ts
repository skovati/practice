import { twoSum } from "./two_sum.ts";
import { assertEquals } from "jsr:@std/assert@1";

Deno.test("one", () => {
    assertEquals(twoSum([3, 4, 5, 6], 7), [0, 1])
})

Deno.test("two", () => {
    assertEquals(twoSum([4, 5, 6], 10), [0, 2])
})
