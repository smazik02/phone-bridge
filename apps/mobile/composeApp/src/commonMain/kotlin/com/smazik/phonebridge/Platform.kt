package com.smazik.phonebridge

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform