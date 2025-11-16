import XCTest
import SwiftTreeSitter
import TreeSitterRanni

final class TreeSitterRanniTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_ranni())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Ranni grammar")
    }
}
