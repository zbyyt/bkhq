#!/bin/env bash
# =========================================
# Author: Iarom Madden

# Rsync Automation
#

# ============================================
#set -e -o pipefail
rep="$1"
cmd="$2"
shift 2
args=()
inc="$XDG_BIN_HOME/fbk/xfr.inc"
inc_part_mid="$XDG_BIN_HOME/fbk/xfr.inc.part"
inc_part_min="$XDG_BIN_HOME/fbk/xfr.inc.part.min"
inc_part_mst="$XDG_BIN_HOME/fbk/xfr.inc.part.mst"
exl_dt="$XDG_BIN_HOME/fbk/xfr.exl"
exl_sy="$XDG_BIN_HOME/fbk/xfr.exly.sy"
rst_mnt="$HOME/zs/xbk.rst/"
hostname="$HOST"


# rep ##################################################A

case $rep in
	dva) 	dst="/home/iao/zm/dva/.rsy" ;;
	dvb) 	dst="/home/iao/zm/dva/.rsy" ;;
	*)		printf "Enter Repo\n" && exit
esac


# src #################################################

case $src in
	home|h)	src="/home/" ;;
	part.mid|pmid) src="$(cat $XDG_BIN_HOME/fbk/xfr.inc.part)" ;;
		# check that they will all have '/' appended to end of name?
	
	full|f) 		src="/data/" ;;
	system|sys) src="/" && bksy.pre

esac


# cmd ################################################
case $cmd in 
	push)
		printf "%s\n" \
			"Push sync (delete) $src to $dst" \
			"---------------------------"
		args+=( '-aAXHvP' )
		args+=( '--delete' )
		args+=( '--delete-excluded' )
		args+=( '--exclude-from=' "$exl_dt")
		args+=( "$@" )
		args+=( "$src" )
		args+=( "$dst" )
		args+=( '2> rerr' )
		printf "%s\n"	\
			" rsync arguments:" \
			"   ${args[@]}" \
			" ----------------------" ;;

	push.avP )
		printf "TODO\n" && exit
		
		args+=( '-avP' ) 
		args+=( "$@" )
			--delete \
			--delete-excluded \
			--exclude-from="$XDG_BIN_HOME/fbk/xfr.exl" \
			$src \
			$dst ;;
	

	push.ls)
		printf "TODO\n" && exit

		rsls  \
		"$@"
			$src_root \
			$src_ls_file \
			$dst_root ;;
	
	
	dvb.part.min|dvb.pmin)
		printf "TODO\n" && exit
		
		#src_root="/data/"
		"$@"
			$src_root \
			$src_ls_file \
			$dst_root \
			$opt ;;

	push.sy)
		printf "TODO\n" && exit
		bksy.pre
			-aAXHvP $opt \
			--delete \
			--delete-excluded \
			--exclude-from="/home/iao/.local/bin/bkexl.sy" \
			"/" \
			"$dst" \
			 2> ~/.cache/rs.bksy.err ;;
	
	ls)
		printf "TODO\n" && exit

		src_root="$1"
		src_ls_file="$2"
		dst_root="$3"
		opt="$4"
		## fc ==============
		rs_for() {
			src_root="$1"
			src_ls_file="$2"
			dst_root="$3"
			opt="$4"

			src_ls=$(cat $src_ls_file)
			cd $src_root
			
			for src_i in $src_ls; do
			  src=$(realpath $src_i)
			  dst="$dst_root/$src_i"
			  if [ ! -d $dst ]; then
			    mkdir $dst
		    	   fi
			  if [ -d $src ]; then
			    rsync \
			    	-avP $opt \
				--delete \
				--links \
				--delete-excluded \
				--exclude-from="$exl" \
			    	"$src/" \
				"$dst/"
			  fi
			done
		}
		## exc ===========
		rs_for $src_root \
			$src_ls_file \
			$dst_root \
			$opt

	;;
esac	

rsync "${args[@]}"


