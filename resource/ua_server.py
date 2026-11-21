#!/usr/local/bin/python
# -*- coding: utf-8 -*-
'''ua_server
GET / HTTP/1.1
Host: localhost
User-Agent: Mozzila/5.0 (OS) Gecko/0 Firefox/0
Custom-Header: custom

body...
'''

from http.server import BaseHTTPRequestHandler, HTTPServer

class UAHandler(BaseHTTPRequestHandler):
  def do_GET(self):
    ua = self.headers.get('User-Agent', '(None)')
    self.send_response(200)
    self.send_header('Content-Type', 'text/plain; charset=utf-8')
    self.end_headers()
    self.wfile.write(f'{ua}\n'.encode('utf-8'))
    self.wfile.flush()
    for k in self.headers:
      self.log_message(f'{k}: [{self.headers[k]}]')

if __name__ == '__main__':
  HTTPServer(('127.0.0.1', 8080), UAHandler).serve_forever()
