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
    
    internal static func get(filePath: URL) throws -> String {
        guard let inputFile = String(data: try Data(contentsOf: filePath), encoding: .utf8) else {
            throw InputError.noFile
        }
        return inputFile
    }
}
