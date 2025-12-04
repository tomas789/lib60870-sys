// lib60870-sys wrapper header
// This file includes all public API headers from lib60870-C

// HAL headers (must come first for type definitions)
#include "hal_base.h"
#include "hal_time.h"
#include "hal_thread.h"
#include "hal_socket.h"
#include "hal_serial.h"
#include "tls_config.h"

// API headers
#include "iec60870_common.h"
#include "cs104_connection.h"
#include "cs104_slave.h"
#include "cs101_master.h"
#include "cs101_slave.h"
#include "link_layer_parameters.h"

