//
//  InputHelpers.swift
//  
//
//  Created by Nick McGuire on 2022-12-02.
//

import Foundation

struct Input {
    private enum InputError: Error {
        case noFile
    }
    
    /// 
    internal static func get(_ path: String, initialPath: String = #file) throws -> String {
        let inputPath = URL(fileURLWithPath: initialPath).deletingLastPathComponent().appending(path: path)
        guard let inputFile = String(data: try Data(contentsOf: inputPath), encoding: .utf8) else {
            throw InputError.noFile
        }
        return inputFile
    }
}
