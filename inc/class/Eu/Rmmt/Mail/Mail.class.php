<?php
/**
 * Fichier de classe
 *
 * PHP version 5.3
 *
 * This file is part of Rendsmoimatune.
 *
 * Rendsmoimatune is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rendsmoimatune is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rendsmoimatune.  If not, see <http://www.gnu.org/licenses/>.
 *
 * @category ClassFile
 * @package  Rendsmoimatune
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @version  SVN: $revision$
 * @link
 */

namespace Eu\Rmmt\Mail;
use Eu\Rmmt\User;
use Bdf\Core;
use Bdf\Utils;

/**
 * Mail
 *
 * @category Class
 * @package
 * @author   needle
 * @license  http://www.gnu.org/copyleft/gpl.html  GPL License 3.0
 * @link
 */
class Mail
{
    private $_toDisplayName;
    private $_toAddress;

    private $_fromDisplayName;
    private $_fromAddress;

    private $_replyToDisplayName;
    private $_replyToAddress;

    private $_messageId;
    private $_subject;
    private $_message;
    private $_html = true;

    const EOL = "\r\n";

    protected function  __construct(User $to, $subject, $message)
    {
        /**
         * Default values
         */
        $this->_mailDomain = Core::getInstance()->getConfig('site', 'mail_domain');

        $this->_fromDisplayName = Utils::getText('%s No reply', Core::getInstance()->getConfig('site', 'site_name'));
        $this->_fromAddress = "no-reply@".$this->_mailDomain;

        $this->_replyToDisplayName = Utils::getText('%s No reply', Core::getInstance()->getConfig('site', 'site_name'));
        $this->_replyToAddress = "no-reply@".$this->_mailDomain;

        /**
         * User defined values
         */
        $this->_toDisplayName = $to->getName();
        $this->_toAddress = $to->getEmail();
        $this->_subject = $subject;
        $this->_message = $message;
    }

    protected function setTo(User $to)
    {
        $this->_toDisplayName = $to->getName();
        $this->_toAddress = $to->getEmail();
    }

    protected function setFrom($displayName, $address)
    {
        $this->_fromDisplayName  = $displayName;
        $this->_fromAddress      = $address;
    }

    protected function setReplyTo($displayName, $address)
    {
        $this->_replyToDisplayName  = $displayName;
        $this->_replyToAddress      = $address;
    }

    private function _generateMessageId()
    {
        mt_srand();
        return '<'.base64_encode(microtime()).'.'.base64_encode(mt_rand()).'@'.$this->_mailDomain.'>';
    }

    private function _utf8HeaderEncode($string)
    {
        return "=?utf-8?B?".base64_encode($string)."?=";
    }

    private function _generateHtmlMessage()
    {
        $message = Utils::replace_uri($this->_message, '<a href="$0">$0</a>');
        $message = nl2br($message);
        $te = Core::getInstance()->getTemplatesEngine();
        $te->assign("message", $message);

        return $te->fetch("mail");
    }

    public function send()
    {
        $to = $this->_utf8HeaderEncode($this->_toDisplayName)." <".$this->_toAddress.">";
        $subject = $this->_utf8HeaderEncode($this->_subject);

        $additional_headers  = "From: ".$this->_utf8HeaderEncode($this->_fromDisplayName)." <".$this->_fromAddress.">".self::EOL;
        $additional_headers .= "Reply-to: ".$this->_utf8HeaderEncode($this->_replyToDisplayName)." <".$this->_replyToAddress.">".self::EOL;
        $additional_headers .= "Message-ID: ".$this->_generateMessageId().self::EOL;
        $additional_headers .= "MIME-Version: 1.0".self::EOL;

        if ($this->_html) {
            mt_srand();
            $boundary = md5(mt_rand());
            $additional_headers .= "Content-type: multipart/alternative; boundary=".$boundary.self::EOL;

            $message = "--".$boundary.self::EOL;
            $message .= "Content-type: text/plain; charset=utf-8".self::EOL.self::EOL;
            $message .= wordwrap($this->_message, 70);

            $message .= self::EOL;

            $message .= "--".$boundary.self::EOL;
            $message .= "Content-type: text/html; charset=utf-8".self::EOL.self::EOL;
            $message .= wordwrap($this->_generateHtmlMessage(), 70);
        } else {
            $additional_headers .= "Content-type: text/plain; charset=utf-8".self::EOL;
            $message = $this->_message;
        }

        mail($to, $subject, $message, $additional_headers);
    }
}
