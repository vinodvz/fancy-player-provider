use std::io::{self, Write};
use std::os::unix::io::FromRawFd;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use byteorder::{ByteOrder, LittleEndian};

#[allow(mutable_static)]
static mut current_session_id: String = String::new();
static mut time_update_cnt: i32 = 0;

fn request_deviceinfo(mut stream: &UnixStream) {
   let request_payload = "{\"method\":\"DeviceInfo\",\"params\":{}}";

   let payload_size = request_payload.len() as u32;
   let mut header:u32 = 0x01000000; //Version
   header |= 0x00000000; //Request
   header |= payload_size & 0x0000FFFF;
   let mut header_buf = [0u8; 4];
   LittleEndian::write_u32(&mut header_buf, header);

   let _ = stream.write_all(&header_buf);
   let _ = stream.write_all(&request_payload.as_bytes());
}

fn get_data_from_store(mut stream: &UnixStream) {
   let request_payload = "{\"method\":\"GetDataFromStore\",\"params\":{}}";

   let payload_size = request_payload.len() as u32;
   let mut header:u32 = 0x01000000; //Version
   header |= 0x00000000; //Request
   header |= payload_size & 0x0000FFFF;
   let mut header_buf = [0u8; 4];
   LittleEndian::write_u32(&mut header_buf, header);

   let _ = stream.write_all(&header_buf);
   let _ = stream.write_all(&request_payload.as_bytes());
}

fn set_data_to_store(mut stream: &UnixStream) {
   let request_payload = "{\"method\":\"SetDataToStore\",\"params\":{\"value\":\"{\\\"key1\\\":1,\\\"key2\\\":\\\"value 2\\\"}\"}}";

   let payload_size = request_payload.len() as u32;
   let mut header:u32 = 0x01000000; //Version
   header |= 0x00000000; //Request
   header |= payload_size & 0x0000FFFF;
   let mut header_buf = [0u8; 4];
   LittleEndian::write_u32(&mut header_buf, header);

   let _ = stream.write_all(&header_buf);
   let _ = stream.write_all(&request_payload.as_bytes());
}

fn sendevent_onscreenmsg(mut stream: &UnixStream) {
   let request_payload = "{\"method\":\"OnScreenMsg\",\"params\":{\"msg\":\"Channelprovider v0.1\\n\\n Sample Log line-1\\n Sample Log line-2\\n Sample Log line-3\"}}";

   let payload_size = request_payload.len() as u32;
   let mut header:u32 = 0x01000000; //Version
   header |= 0x00020000; //Event
   header |= payload_size & 0x0000FFFF;
   let mut header_buf = [0u8; 4];
   LittleEndian::write_u32(&mut header_buf, header);

   println!("Sending Event OnScreenMsg");
   let _ = stream.write_all(&header_buf);
   let _ = stream.write_all(&request_payload.as_bytes());
}

fn send_token_refresh(mut stream: &UnixStream) {
   //Sending the event OnChannelTokenRefresh
   let url = "https://231ff43b53054eefafeaaf1a6c5dcbf8.mediatailor.us-east-1.amazonaws.com/v1/master/0fb304b2320b25f067414d481a779b77db81760d/Vizio_FuboSportsNetwork/playlist.m3u8?ads.wurl_channel=1358&ads.wurl_name=FuboSportsNetwork&ads.coppa=0&ads.psid=%7bADID%7d&ads.targetopt=%7bTARGETOPT%7d&ads.us_privacy=%7bUSPRIVACY%7d&ads.ifa_type=%7bIFATYPE%7d&ads.lmt=%7bLMT%7d&ads.app_bundle=%7bAPP_BUNDLE%7d&ads.app_name=%7bAPP_NAME%7d&ads.app_store_url=%7bAPP_STORE_URL%7d&ads.url=%7bDOMAIN%7d&ads.w=%7bWIDTH%7d&ads.h=%7bHEIGHT%7d&ads.device_make=%7bDEVICE_MAKE%7d&ads.vizdartoken=eyJhdWQiOiI5OCIsIndmcGNpZCI6Mzk1ODAzODMxMH0";
   let key_system = "CLEAR_KEY";
   let license_server_url = "";
   let mut response_payload = "{\"method\":\"OnChannelTokenRefresh\", \"session_id\":\"".to_owned();
   unsafe {
      response_payload += &current_session_id;
   }
   response_payload += "\",\"params\":{\"url\":\"";
   response_payload += url;
   response_payload += "\", \"drm_config\":{\"keySystem\":\"";
   response_payload += key_system;
   response_payload += "\", \"licenseServerUrl\":\"";
   response_payload += license_server_url;
   response_payload += "\"}";
   response_payload += ", \"debug\":false}}";

   let payload_size = response_payload.len() as u32;
   let mut header:u32 = 0x01000000; //Version
   header |= 0x00020000; //Event
   header |= payload_size & 0x0000FFFF;
   let mut header_buf = [0u8; 4];
   LittleEndian::write_u32(&mut header_buf, header);
   println!("Sending OnChannelTokenRefresh to Fancy: header:{:x} payload_len:{payload_size} payload:{response_payload}", header);
   let _ = stream.write_all(&header_buf);
   let _ = stream.write_all(&response_payload.as_bytes());
}

fn handle_request(mut stream: &UnixStream, payload: &str, shutdown: &mut bool) {
   let parsed_json = json::parse(payload).unwrap();

   println!("Received Request : {0}", parsed_json["method"]);
   if parsed_json["method"] == "OpenChannel" {
      //Handle OpenChannel
      unsafe {
         current_session_id = parsed_json["session_id"].to_string();
      }
      let url = "https://cdn-ue1-prod.tsv2.amagi.tv/linear/vizioAAAA-blueantmedia-usa-vizio/playlist.m3u8?did=%7bADID%7d&us_privacy=%7bUSPRIVACY%7d&ifa_type=%7bIFATYPE%7d&lmt=%7bLMT%7d&app_name=%7bAPP_NAME%7d&app_bundle=%7bAPP_BUNDLE%7d&app_store_url=%7bAPP_STORE_URL%7d&url=%7bDOMAIN%7d&dnt=%7bDNT%7d&coppa=%7bCOPPA%7d&device_make=%7bDEVICE_MAKE%7d&w=%7bWIDTH%7d&h=%7bHEIGHT%7d&skip=%7bSKIPPABLE%7d&vizdartoken=eyJhdWQiOiI5MyIsIndmcGNpZCI6MzIwNzc5Mjc4OH0";
      let key_system = "CLEAR_KEY";
      let license_server_url = "";
      let mut response_payload = "{\"method\":\"OpenChannel\", \"session_id\":\"".to_owned();
      response_payload += &parsed_json["session_id"].to_string();
      response_payload += "\", \"status\":\"success\", ";
      response_payload += "\"params\":{\"url\":\"";
      response_payload += url;
      response_payload += "\", \"drm_config\":{\"keySystem\":\"";
      response_payload += key_system;
      response_payload += "\", \"licenseServerUrl\":\"";
      response_payload += license_server_url;
      response_payload += "\"}";
      response_payload += ", \"debug\":false}}";

      let payload_size = response_payload.len() as u32;
      let mut header:u32 = 0x01000000; //Version
      header |= 0x00010000; //Response
      header |= payload_size & 0x0000FFFF;
      let mut header_buf = [0u8; 4];
      LittleEndian::write_u32(&mut header_buf, header);
      println!("Sending to Fancy: header:{:x} payload_len:{payload_size} payload:{response_payload}", header);
      let _ = stream.write_all(&header_buf);
      let _ = stream.write_all(&response_payload.as_bytes());
   } else if parsed_json["method"] == "Shutdown" {
      *shutdown = true;
   }
}

fn handle_response(payload: &str) {
   let parsed_json = json::parse(payload).unwrap();
   
   println!("Received response for : {0}", parsed_json["method"]);

   //TODO: Handle each responses here

}

fn handle_event(stream: &UnixStream, payload: &str) {
   let parsed_json = json::parse(payload).unwrap();
   
   println!("Received Event : {0}", parsed_json["method"]);

   if parsed_json["method"] == "OnBuffering" {
      //TODO: Handle OnBuffering
   } else if parsed_json["method"] == "OnTimeUpdate" {
      unsafe {
         time_update_cnt += 1;
         if time_update_cnt == 40 {
            send_token_refresh(stream);
         }
      }
      //TODO: Handle OnTimeUpdate
   } else if parsed_json["method"] == "OnTimedMetadata" {
      //TODO: Handle OnTimedMetadata
   } else if parsed_json["method"] == "OnError" {
      //TODO: Handle OnError
   } else if parsed_json["method"] == "OnFirstFrameDisplayed" {
      //TODO: Handle OnFirstFrameDisplayed
   } else if parsed_json["method"] == "OnResolutionChanged" {
      //TODO: Handle OnResolutionChanged
   } else if parsed_json["method"] == "OnChannelClosed" {
      //TODO: Handle OnChannelClosed
   }
}

fn handle_fancy_player_messages(mut stream: &UnixStream) {
   let mut shutdown: bool =  false;
   while shutdown != true {
      let mut buf = vec![0u8; 4];
      println!("Waiting for data from fancy player.");
      match stream.read_exact(&mut buf) {
         Err(_) => panic!("Couldn't read message."),
         Ok(_) => {
            let buf_as_bytes: [u8; 4] = buf.try_into().unwrap();
            let header = u32::from_ne_bytes(buf_as_bytes);
            let version = header >> 24;
            let flags = (header & 0x00FF0000) >> 16;
            let payload_len:usize = (header & 0x0000FFFF) as usize;

            buf = vec![0u8; payload_len];
            _ = stream.read_exact(&mut buf);
            let payload = std::str::from_utf8(&buf).unwrap();
            println!("Received from Fancy: version:{:x} flags:{:x} payload_len:{:x} payload:{payload}", version, flags, payload_len);

            if 0x00 == flags {
               //Request
               handle_request(stream, payload, &mut shutdown);
            } else if 0x01 == (flags & 0x03) {
               //Response
               handle_response(payload);
            } else if 0x02 == (flags & 0x03) {
               //Events
               handle_event(stream, payload);
            }
         }
      }
   }
}

fn main() {
   let stream = unsafe { UnixStream::from_raw_fd(0) };
   drop(io::stdin());
   request_deviceinfo(&stream);
   sendevent_onscreenmsg(&stream);
   get_data_from_store(&stream);
   set_data_to_store(&stream);
   handle_fancy_player_messages(&stream);
   drop(stream);
   println!("Exiting Channel provider gracefully.");
}

