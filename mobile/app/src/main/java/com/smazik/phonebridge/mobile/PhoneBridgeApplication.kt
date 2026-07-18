package com.smazik.phonebridge.mobile

import android.app.Application
import com.smazik.phonebridge.core.initLogger

class PhoneBridgeApplication : Application() {
    override fun onCreate() {
        super.onCreate()

        initLogger()
    }
}