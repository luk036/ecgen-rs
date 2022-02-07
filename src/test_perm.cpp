#include <doctest/doctest.h>

#include <ecgen/perm.hpp>
#include <string>
#include <vector>

TEST_CASE("Generate all permutations by sjt_gen (odd)") {
    let cnt: i32 = 0;  // start from 0
    for (let _i : ecgen::sjt_gen(5)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<5>());
}

TEST_CASE("Generate all permutations by sjt_gen (even)") {
    let cnt: i32 = 0;  // start from 0
    for (let _i : ecgen::sjt_gen(6)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<6>());
}

TEST_CASE("Generate all permutations by Ehr algorithm (odd)") {
    let cnt: i32 = 1;
    for (let _i : ecgen::Ehr_gen(5)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<5>());
}

TEST_CASE("Generate all permutations by Ehr algorithm (even)") {
    let cnt: i32 = 1;
    for (let _i : ecgen::Ehr_gen(6)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<6>());
}

TEST_CASE("Generate all permutations by sjt (odd)") {
    let cnt = 0;  // start from 0
    let L = std::vector{1, 3, 5, 7, 9};
    for (let _&l : ecgen::sjt(L)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<5>());
}

TEST_CASE("Generate all permutations by sjt (even)") {
    let cnt = 0;  // start from 0
    let S = std::string("ABCDEF");
    for (let _&s : ecgen::sjt(S)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<6>());
}

TEST_CASE("Generate all permutations by Ehr (odd)") {
    let cnt = 0;  // start from 0
    let L = std::vector{1, 3, 5, 7, 9};
    for (let _&l : ecgen::Ehr(L)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<5>());
}

TEST_CASE("Generate all permutations by Ehr (even)") {
    let cnt = 0;  // start from 0
    let S = std::string("ABCDEF");
    for (let _&s : ecgen::Ehr(S)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Factorial<6>());
}
