// this should probably be codegenned but whatever..
// https://github.com/Hamlib/Hamlib/blob/master/src/token.h
// modified with search: #define ([^\s]+) +TOKEN_FRONTEND\((.+?(?=\)))\)
// vscode        replace: pub const $1: i64 = $2 | (1 << 30);

// original file header:
/*
 *  Hamlib Interface - token header
 *  Copyright (c) 2000-2009 by Stephane Fillod
 *
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Lesser General Public
 *  License as published by the Free Software Foundation; either
 *  version 2.1 of the License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 *  Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public
 *  License along with this library; if not, write to the Free Software
 *  Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 *
 */


/* Null frontend token */
pub const TOK_FRONTEND_NONE: i64 = 0;
/* Null backend token */
pub const TOK_BACKEND_NONE: i64 = 0;

/*
 * tokens shared among rig and rotator,
 * Numbers go from TOKEN_FRONTEND(1) to TOKEN_FRONTEND(99)
 */

/* Pathname is device for rig control, e.g. /dev/ttyS0 */
pub const TOK_PATHNAME: i64 = 10 | (1 << 30);
/* Delay before serial output (units?) */
pub const TOK_WRITE_DELAY: i64 = 12 | (1 << 30);
/* Delay after serial output (units?) */
pub const TOK_POST_WRITE_DELAY: i64 = 13 | (1 << 30);
/* Timeout delay (units?) */
pub const TOK_TIMEOUT: i64 = 14 | (1 << 30);
/* Number of retries permitted */
pub const TOK_RETRY: i64 = 15 | (1 << 30);
/* Serial speed - "baud rate" */
pub const TOK_SERIAL_SPEED: i64 = 20 | (1 << 30);
/* No. data bits per serial character */
pub const TOK_DATA_BITS: i64 = 21 | (1 << 30);
/* No. stop bits per serial character */
pub const TOK_STOP_BITS: i64 = 22 | (1 << 30);
/* Serial parity (format?) */
pub const TOK_PARITY: i64 = 23 | (1 << 30);
/* Serial Handshake (format?) */
pub const TOK_HANDSHAKE: i64 = 24 | (1 << 30);
/* Serial Req. To Send status */
pub const TOK_RTS_STATE: i64 = 25 | (1 << 30);
/* Serial Data Terminal Ready status */
pub const TOK_DTR_STATE: i64 = 26 | (1 << 30);
/* PTT type override */
pub const TOK_PTT_TYPE: i64 = 30 | (1 << 30);
/* PTT pathname override */
pub const TOK_PTT_PATHNAME: i64 = 31 | (1 << 30);
/* DCD type override */
pub const TOK_DCD_TYPE: i64 = 32 | (1 << 30);
/* DCD pathname override */
pub const TOK_DCD_PATHNAME: i64 = 33 | (1 << 30);
/* CM108 GPIO bit number for PTT */
pub const TOK_PTT_BITNUM: i64 = 34 | (1 << 30);
/* PTT share with other applications */
pub const TOK_PTT_SHARE: i64 = 35 | (1 << 30);
/* Flush with read instead of TCFLUSH */
pub const TOK_FLUSHX: i64 = 36 | (1 << 30);
/* Asynchronous data transfer support */
pub const TOK_ASYNC: i64 = 37 | (1 << 30);
/* Tuner external control pathname */
pub const TOK_TUNER_CONTROL_PATHNAME: i64 = 38 | (1 << 30);
/* Number of retries permitted in case of read timeouts */
pub const TOK_TIMEOUT_RETRY: i64 = 39 | (1 << 30);
pub const TOK_POST_PTT_DELAY: i64 = 40 | (1 << 30);
pub const TOK_DEVICE_ID: i64 = 41 | (1 << 30);

/*
 * rig specific tokens
 */
/* rx_range_list/tx_range_list, filters, announces, has(func,lvl,..) */

/* rig: VFO compensation in ppm */
pub const TOK_VFO_COMP: i64 = 110 | (1 << 30);
/* rig: Rig state poll interval in milliseconds */
pub const TOK_POLL_INTERVAL: i64 = 111 | (1 << 30);
/* rig: lo frequency of any transverters */
pub const TOK_LO_FREQ: i64 = 112 | (1 << 30);
/* rig: Range index 1-5 */
pub const TOK_RANGE_SELECTED: i64 = 121 | (1 << 30);
/* rig: Range Name */
pub const TOK_RANGE_NAME: i64 = 122 | (1 << 30);
/* rig: Cache timeout in milliseconds */
pub const TOK_CACHE_TIMEOUT: i64 = 123 | (1 << 30);
/* rig: Auto power on rig_open when supported */
pub const TOK_AUTO_POWER_ON: i64 = 124 | (1 << 30);
/* rig: Auto power off rig_close when supported */
pub const TOK_AUTO_POWER_OFF: i64 = 125 | (1 << 30);
/* rig: Auto disable screensaver */
pub const TOK_AUTO_DISABLE_SCREENSAVER: i64 = 126 | (1 << 30);
/* rig: Disable Yaesu band select logic */
pub const TOK_DISABLE_YAESU_BANDSELECT: i64 = 127 | (1 << 30);
/* rig: Suppress get_freq on VFOB for satellite RIT tuning */
pub const TOK_TWIDDLE_TIMEOUT: i64 = 128 | (1 << 30);
/* rig: Suppress get_freq on VFOB for satellite RIT tuning */
pub const TOK_TWIDDLE_RIT: i64 = 129 | (1 << 30);
/* rig: Add Hz to VFOA/Main frequency set */
pub const TOK_OFFSET_VFOA: i64 = 130 | (1 << 30);
/* rig: Add Hz to VFOB/Sub frequency set */
pub const TOK_OFFSET_VFOB: i64 = 131 | (1 << 30);
/* rig: Multicast data UDP address for publishing rig data and state, default 224.0.0.1, value of 0.0.0.0 disables multicast data publishing */
pub const TOK_MULTICAST_DATA_ADDR: i64 = 132 | (1 << 30);
/* rig: Multicast data UDP port, default 4532 */
pub const TOK_MULTICAST_DATA_PORT: i64 = 133 | (1 << 30);
/* rig: Multicast command server UDP address for sending commands to rig, default 224.0.0.2, value of 0.0.0.0 disables multicast command server */
pub const TOK_MULTICAST_CMD_ADDR: i64 = 134 | (1 << 30);
/* rig: Multicast command server UDP port, default 4532 */
pub const TOK_MULTICAST_CMD_PORT: i64 = 135 | (1 << 30);
/* rig: Skip setting freq on opposite VFO when in split mode */
pub const TOK_FREQ_SKIP: i64 = 136 | (1 << 30);
/* rig: Client ID of WSJTX or GPREDICT */
pub const TOK_CLIENT: i64 = 137 | (1 << 30);

/*
 * rotator specific tokens
 * (strictly, should be documented as rotator_internal)
 */
/* rot: Minimum Azimuth */
pub const TOK_MIN_AZ: i64 = 110 | (1 << 30);
/* rot: Maximum Azimuth */
pub const TOK_MAX_AZ: i64 = 111 | (1 << 30);
/* rot: Minimum Elevation */
pub const TOK_MIN_EL: i64 = 112 | (1 << 30);
/* rot: Maximum Elevation */
pub const TOK_MAX_EL: i64 = 113 | (1 << 30);
/* rot: South is zero degrees */
pub const TOK_SOUTH_ZERO: i64 = 114 | (1 << 30);