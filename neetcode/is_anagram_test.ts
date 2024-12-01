import { isAnagram } from "./is_anagram.ts";
import { assertEquals } from "jsr:@std/assert@1";

Deno.test("one", () => {
    assertEquals(isAnagram("racecar", "carrace"), true)
})

Deno.test("two", () => {
    assertEquals(isAnagram("jam", "jar"), false)
})
