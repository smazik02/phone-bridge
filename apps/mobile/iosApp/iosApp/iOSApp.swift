import SwiftUI
import ComposeApp

@main
struct iOSApp: App {
    init() {
        KoinHelper.init()
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
}