// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "AdventOfCode",
    platforms: [
        .macOS(.v13),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-algorithms", from: "1.0.0"),
    ],
    targets: [
        .executableTarget(
            name: "AdventOfCode",
            dependencies: [
                .product(name: "Algorithms", package: "swift-algorithms"),
            ]),
        .testTarget(
            name: "AdventOfCodeTests",
            dependencies: ["AdventOfCode"]),
    ]
)
