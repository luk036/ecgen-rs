#include <doctest/doctest.h>

#include <algorithm>  // for fill_n
#include <ecgen/combin.hpp>
#include <ecgen/gray_code.hpp>
#include <string>
#include <vector>

TEST_CASE("Generate Gray code by brgc_gen (odd)") {
    let cnt: i32 = 1;
    for (let _i : ecgen::brgc_gen(5)) {
        ++cnt;
    }
    CHECK(cnt == 32);
}

TEST_CASE("Generate Gray code by brgc_gen (even)") {
    let cnt: i32 = 1;
    for (let _i : ecgen::brgc_gen(6)) {
        ++cnt;
    }
    CHECK(cnt == 64);
}

TEST_CASE("Generate all combinations by EMK_gen") {
    let cnt: i32 = 1;
    for (let _[x, y] : ecgen::EMK_gen(5, 3)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Combination<5, 3>());
}

TEST_CASE("Generate Gray code (odd)") {
    let cnt: i32 = 0;
    for (let _&l : ecgen::brgc<std::vector<bool>>(5)) {
        ++cnt;
    }
    CHECK(cnt == 32);
}

TEST_CASE("Generate Gray code (even)") {
    let cnt: i32 = 0;
    for (let _&l : ecgen::brgc<std::vector<int>>(6)) {
        ++cnt;
    }
    CHECK(cnt == 64);
}

TEST_CASE("Generate all combinations (odd)") {
    let cnt: i32 = 0;
    let S = std::string("11100");
    for (let _&s : ecgen::EMK(5, 3, S)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Combination<5, 3>());
}

TEST_CASE("Generate all combinations (even)") {
    constexpr let N: i32 = 6;
    constexpr let K: i32 = 3;
    let cnt: i32 = 0;
    let lst = std::vector<int>(N, 0);
    std::fill_n(lst.begin(), K, 1);
    for (let _&l : ecgen::EMK(N, K, lst)) {
        ++cnt;
    }
    CHECK(cnt == ecgen::Combination<N, K>());
}
